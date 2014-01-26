#[crate_id = "radicle"];

//! A lisp interpreter.


use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

use std::hashmap::HashMap;
use tree::Tree;
use Atom = tree::Leaf;
use List = tree::Branch;


pub mod tree;

fn main() {
    let globenv = Environment { parent: None, bindings: HashMap::new() };
    read_eval("(quote x)", &globenv);
    read_eval("(atom x)", &globenv);
    read_eval("(atom (quote x))", &globenv);
    read_eval("(atom (atom x))", &globenv);
    read_eval("(atom (quote ()))", &globenv);
    read_eval("(atom (my little pony))", &globenv);
    read_eval("(atom (quote (my little pony)))", &globenv);
    read_eval("(car (quote (10 5 9)))", &globenv);
    read_eval("(cdr (quote (10)))", &globenv);
    read_eval("(cdr (quote (10 5 9)))", &globenv);
    read_eval("(cons (quote 7) (quote (10 5 9)))", &globenv);
    read_eval("(cons (quote 7) (quote ()))", &globenv);
    read_eval("(car (cdr (quote (1 t 3))))", &globenv);
    read_eval("(cond ((quote f) 7) ((quote foo) 8) ((quote t) (quote 9)))", &globenv);
    read_eval("(cond ((quote (1 t 3)) 7) ((car (quote (1 t 3))) 8) ((car (cdr (quote (1 t 3)))) (quote (a b c))))", &globenv);
    read_eval("((lambda (x) (cons x (quote (ab cd)))) (quote CONSME))", &globenv);
    read_eval("((lambda (x y z) (cons y (cons z (cons x (quote (batman)))))) (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);
    read_eval("((lambduh (x) (cons x (quote ()))) (quote CONSME))", &globenv);
    read_eval("(((lambda (x) 
           (lambda (y) (cons x 
                             (cons y 
                                   (quote ()))))) 
   (quote 5)) (quote 6))", &globenv);

}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: &Environment) {
    println!("input: {}", s);
    let parsed = read(s);
    if parsed.is_ok() {
        println!("Parsed: {}", parsed);
        match eval(parsed.unwrap(), env) {
            Ok(x) => { println!("evaled: {}", x); },
            Err(x) => { println!("Eval error: {}", x); }
        }
    } else {
        println!("Parse error: {}", parsed.unwrap_err());
    }
    println!("-----------");
}


/// Intermediate representation after tokenization and before it gets read into
/// and expression.
pub type TokenStream = Peekable<~str, MoveItems<~str>>;

/// The representation of Lisp expressions
pub type Expression = Tree<~str>;


struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    bindings: HashMap<~str, Expression>,
}

impl<'a> Environment<'a> {
    fn find(&'a self, key: &~str) -> Option<&'a Expression> {
        if self.bindings.contains_key(key) {
            self.bindings.find(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find(key)
            } else {
                None
            }
        }
    }

    fn find_copy(&self, key: &~str) -> Option<Expression> {
        if self.bindings.contains_key(key) {
            self.bindings.find_copy(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find_copy(key)
            } else {
                None
            }
        }
    }
}

/// Wrapping the standard Tree methods for aesthetic reasons, I guess
impl ::tree::Tree<~str> {
    fn is_atom(&self) -> bool {
        self.is_leaf()
    }

    fn is_list(&self) -> bool {
        self.is_branch()
    }
}


/// Reads a string of symbols into an expression (or possibly an error message)
pub fn read(s: &str) -> Result<Expression, &str> {
    let mut stream = tokenize(s);
    let x = read_from(&mut stream);

    // eventually this will be stream.is_empty(), but theres a bug rust!
    if stream.peek().is_some() {
        return Err("Tokens left over, so parse was unsuccessful.");
    }

    x
}


/// Turns a string into a stream of tokens. Currently assumes that tokens
/// do not have spaces or parens in them.
pub fn tokenize(s: &str) -> TokenStream {
    let s1 = s.replace("(", " ( ").replace(")", " ) ");

    let x: ~[&str] = s1.split(|c: char| is_whitespace(c)).collect();
    
    let mut ret: ~[~str] = ~[];
    for &e in x.iter() {
        if e != "" {
            ret.push(e.to_owned());
        }
    }

    ret.move_iter().peekable()
}

/// Attempts to read an entire expression from the token stream. Detects
/// mismatched parentheses.
pub fn read_from(v: &mut TokenStream) -> Result<Expression, &str> {
    let tok = v.next();
    match tok {
        None        => Err("Unexpected end of token stream"),
        Some(s) =>
            if "(".equiv(&s) {
                let mut ch = ~[];

                loop {
                    {
                        let x = v.peek();
                        if x.is_some() && ")".equiv(x.unwrap()) {
                            break;
                        }
                    }

                    match read_from(v) {
                        Err(e) => { return Err(e); },
                        Ok(expr) => { ch.push(expr); }
                    }
                }

                v.next();
                Ok( List(ch) )

            } else if ")".equiv(&s) {
                Err("Unexpected ')'")
            } else {
                Ok( Atom(s) )
            }
    }
}


/// The heart and soul of Radicle.
pub fn eval<'a>(expr: Expression, env: &'a Environment<'a>) -> Result<Expression, ~str> {
    match expr {
        Atom(ref s) => {
            let res = env.find_copy(s);
            if res.is_none() {
                Err(format!("Symbol `{}` not found.", *s))
            } else {
                Ok(res.unwrap())
            }
        },
        List([]) => Err(~"No procedure to call. TODO: a better error message?"),
        List(vec) => {
            let t = Atom(~"t");
            let empty: Expression = List(~[]);

            if is_primitive_op("quote", &vec[0]) {

                if vec.len() != 2 {
                    Err(~"`quote` expects exactly one argument.")
                } else {
                    Ok( vec[1] )
                }

            } else if is_primitive_op("atom", &vec[0]) {

                let eval_atom = |val: Expression| -> Result<Expression, ~str> {
                    if val.is_atom() || val.eq(&empty) {
                        Ok( t.clone() )
                    } else {
                        Ok( empty.clone() )
                    }
                };

                if vec.len() != 2 {
                    Err(~"`atom` expects exactly one argument.")
                } else {
                    result_bind(eval(vec[1], env), eval_atom)
                }

            } else if is_primitive_op("eq", &vec[0]) {

                let eval_eq = |val1: Expression, val2: Expression| 
                              -> Result<Expression, ~str> {
                    if (val1.eq(&empty) && val2.eq(&empty))
                       || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
                        Ok( t.clone() )
                    } else {
                        Ok( empty.clone() )
                    }
                };

                if vec.len() != 3 {
                    Err(~"`eq` expects exactly two arguments.")
                } else {
                    result_fmap2(eval(vec[1].clone(), env), eval(vec[2], env), eval_eq)
                }

            } else if is_primitive_op("car", &vec[0]) {

                let eval_car = |val: Expression| -> Result<Expression, ~str> {
                    if val.is_list() && !val.eq(&empty) {
                        let list = val.unwrap_branch();
                        Ok( list[0] )
                    } else {
                        Err(~"`car`'s argument must be a non-empty list")
                    }
                };

                if vec.len() != 2 {
                    Err(~"`car` expects exactly one argument.")
                } else {
                    result_bind(eval(vec[1], env), eval_car)
                }

            } else if is_primitive_op("cdr", &vec[0]) {

                let eval_cdr = |val: Expression| -> Result<Expression, ~str> {
                    if val.is_list() && !val.eq(&empty) {
                        let mut list = val.unwrap_branch();
                        list.shift();
                        Ok( List(list) )
                    } else {
                        Err(~"`cdr`'s argument must be a non-empty list")
                    }
                };

                if vec.len() != 2 {
                    Err(~"`cdr` expects exactly one argument.")
                } else {
                    result_bind(eval(vec[1], env), eval_cdr)
                }

            } else if is_primitive_op("cons", &vec[0]) {

                let eval_cons = |val1: Expression, val2: Expression| 
                              -> Result<Expression, ~str> {
                    if val2.is_list() {
                        let mut list = val2.unwrap_branch();
                        list.unshift(val1);
                        Ok( List(list) )
                    } else {
                        Err(~"`cons`'s second argument must be a list")
                    }
                };

                if vec.len() != 3 {
                    Err(~"`cons` expects exactly two arguments.")
                } else {
                    result_fmap2(eval(vec[1].clone(), env), eval(vec[2], env), eval_cons)
                }

            } else if is_primitive_op("cond", &vec[0]) {

                let mut i = 1;
                while i < vec.len() {
                    if !vec[i].is_list() {
                        return Err(~"Invalid argument to `cond`");
                    } 

                    let arg = vec[i].clone();
                    let list = arg.unwrap_branch();
                    
                    if list.len() != 2 {
                        return Err(~"Invalid argument to `cond`");
                    } else {
                        let res = eval(list[0].clone(), env);
                        if res.is_err() {
                            return res;
                        } else {
                            let val = res.unwrap();

                            if val.eq(&t) {
                                return eval(list[1], env);
                            }
                        }
                    }

                    i += 1;
                }

                Err(~"No branch of `cond` evaluated to true. Don't think this is an error, though. Need to decide how to handle.")

            } else {

                let num_args = vec.len() - 1;

                let mut vec_iter = vec.move_iter();
                let mut op_expr = vec_iter.next().unwrap();

                // we need to distinguish between "literal" lambda calls, where
                // the first op literally matches the lambda pattern
                //     (lambda (p1 ... pn) body)
                // and the case where the op expression is something that evaluates
                // to the literal lambda pattern.
                //
                // the reason we need to distinguish is that we must not evaluate
                // the op in the first case, since (lambda (p1 ... pn) body) does
                // not evaluate to anything, according to PG?
                if !is_lambda_literal(&op_expr) {
                    let res = eval(op_expr, env);

                    if res.is_err() {
                        return res;
                    } else {
                        op_expr = res.unwrap();

                        if !is_lambda_literal(&op_expr) {
                            return Err(~"Unrecognized expression.");
                        }
                    }
                }

                // If we've made it here, op_expr is a lambda literal.
                let lambda: ~[Expression] = op_expr.unwrap_branch();

                let mut lambda_iter = lambda.move_iter();
                lambda_iter.next(); // discard "lambda" symbol, not needed

                // params is the list of formal arguments to the lambda
                let params: ~[Expression] = lambda_iter.next().unwrap().unwrap_branch();
                let lambda_body: Expression = lambda_iter.next().unwrap();

                if params.len() != num_args {
                    return Err(~"mismatch between number of procedure args and number of args called with.");
                }

                let mut bindings = HashMap::<~str, Expression>::new();

                // iterate through remaining elements in vec_iter
                // for each element, evaluate it. if we get an error,
                // terminate and return that error.
                // otherwise, insert that vaue into the bindings HashMap that
                // we are building up and continue onto the next arg
                let mut param_iter = params.move_iter();

                for v in vec_iter {
                    let res = eval(v, env);

                    if res.is_err() {
                        return res;
                    } else {
                        bindings.insert(param_iter.next().unwrap().unwrap_leaf(),
                                        res.unwrap());
                    }

                }

                let new_env = Environment { parent: Some(env), 
                                            bindings: bindings };

                return eval(lambda_body, &new_env);
            }
        }
    }
}

fn is_lambda_literal(expr: &Expression) -> bool {
    if !expr.is_list() {
        return false;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_list() 
       || !is_primitive_op("lambda", &vec[0]) {
        return false;
    }

    let params = vec[1].get_ref_branch();

    for p in params.iter() {
        if !p.is_atom() {
            return false;
        }
    }

    true
}

fn is_primitive_op(op: &str, expr: &Expression) -> bool {
    if expr.is_atom() {
        let expr_op = expr.get_ref_leaf();
        op.equiv(expr_op)
    } else {
        false
    }
}


/// A bind for the Result<T, ~str> monad.
pub fn result_bind<S, T>(v: Result<S, ~str>, f: |S| -> Result<T, ~str>) -> Result<T, ~str> {
    match v {
        Err(s) => Err(s),
        Ok(x) => f(x),
    }
}

/// Fmap2 for the Result<T, ~str> monad. Used in a couple places in eval()
pub fn result_fmap2<S, T, U>(v1: Result<S, ~str>, 
                         v2: Result<T, ~str>, 
                         f: |S, T| -> Result<U, ~str>) -> Result<U, ~str> {
    match v1 {
        Err(s) => Err(s),
        Ok(x) => 
            match v2 {
                Err(s) => Err(s),
                Ok(y) => f(x, y),
            }
    }
}
