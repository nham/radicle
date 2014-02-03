#[crate_id = "radicle"];

//! A lisp interpreter.

use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;
use std::hashmap::HashMap;
use std::str;
use std::io::File;
use std::os;

use tree::Tree;
use Atom = tree::Leaf;
use List = tree::Branch;


pub mod tree;
mod test;

fn main() {
    let globenv = Environment { parent: None, bindings: HashMap::new() };

    let args = os::args();
    if args.len() == 1 {
        println!("radicle: REPL is not yet implemented.");
        return;
    } else if args.len() > 2 {
        println!("radicle: Only one argument allowed.");
        return;
    }

    if !"--test".equiv(&args[1]) {
        let fname = args[1].clone();
        let path = Path::new(args[1]);
        if path.is_file() {
            let mut hw_file = File::open(&path);
            let data = str::from_utf8_owned(hw_file.read_to_end());
            read_eval(data.unwrap(), &globenv);
            return;
        } else {
            println!("radicle: can't open file {}", fname);
            return;
        }
    }

    /*
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
    read_eval(
"((label ZABBA (lambda (x) (cons x (quote (ab cd)))))
  (quote CONSME))", &globenv);

    read_eval(
"((label ZABBA (lambda (x y z) (cons y (cons z (cons x (quote (batman)))))))
  (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);


    read_eval("(quote x) (quote y) (quote z)", &globenv);
 */
}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: &Environment) {
    println!("input: {}", s);
    let parsed = read(s);
    if parsed.is_ok() {
        println!("\nParsed: {}", parsed);
        match eval(parsed.unwrap(), env) {
            Ok(x) => { println!("\nevaled: {}", x); },
            Err(x) => { println!("\nEval error: {}", x); }
        }
    } else {
        println!("\nParse error: {}", parsed.unwrap_err());
    }
    println!("\n>>>>>>>>>>>>>>>>\n");
}


/// Intermediate representation after tokenization and before it gets read into
/// and expression.
pub type TokenStream = Peekable<~str, MoveItems<~str>>;

/// The representation of Lisp expressions
pub type Expr = Tree<~str>;


pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    bindings: HashMap<~str, Expr>,
}

impl<'a> Environment<'a> {
    fn find(&'a self, key: &~str) -> Option<&'a Expr> {
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

    fn find_copy(&self, key: &~str) -> Option<Expr> {
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

    fn is_empty_list(&self) -> bool {
        self.eq(&List(~[]))
    }
}


/// Reads a string of symbols into an expression (or possibly an error message)
pub fn read(s: &str) -> Result<Expr, ~str> {
    let mut stream = tokenize(s);
    let x = read_from(&mut stream);

    // eventually this will be stream.is_empty(), but theres a bug rust!
    if stream.peek().is_some() {
        return Err(~"Tokens left over, so parse was unsuccessful.");
    }

    x
}


/// Turns a string into a stream of tokens. Currently assumes that tokens
/// do not have spaces or parens in them.
pub fn tokenize(s: &str) -> TokenStream {
    let mut s1 = s.replace("(", " ( ").replace(")", " ) ");
    s1 = s1.replace("[", " [ ").replace("]", " ] ");
    s1 = s1.replace("{", " { ").replace("}", " } ");

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
pub fn read_from(v: &mut TokenStream) -> Result<Expr, ~str> {
    fn is_beginning_list_sep(s: &~str) -> bool {
        "(".equiv(s) || "[".equiv(s) || "{".equiv(s)
    }

    fn is_ending_list_sep(s: &~str) -> bool {
        ")".equiv(s) || "]".equiv(s) || "}".equiv(s)
    }

    let tok = v.next();
    match tok {
        None        => Err(~"Unexpected end of token stream"),
        Some(s) =>
            if is_beginning_list_sep(&s) {
                let mut ch = ~[];

                loop {
                    {
                        let x = v.peek();
                        if x.is_some() && is_ending_list_sep( x.unwrap()) {
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

            } else if is_ending_list_sep(&s) {
                Err(format!("Unexpected '{}'", s))
            } else if s.starts_with("'") {
                Ok( List(~[Atom(~"quote"), Atom( s.slice_from(1).to_owned() )]) )
            } else {
                Ok( Atom(s) )
            }
    }
}


/// The heart and soul of Radicle.
pub fn eval<'a>(expr: Expr, env: &'a Environment<'a>) -> Result<Expr, ~str> {
    debug!("\n :: Entered eval, expr = \n{}\n", expr);
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
            if is_symbol("quote", &vec[0]) {
                if vec.len() != 2 {
                    Err(~"`quote` expects exactly one argument.")
                } else {
                    Ok( vec[1] )
                }
            } else if is_symbol("atom", &vec[0]) {
                eval_atom(vec, env)
            } else if is_symbol("eq", &vec[0]) {
                eval_eq(vec, env)
            } else if is_symbol("car", &vec[0]) {
                eval_car(vec, env)
            } else if is_symbol("cdr", &vec[0]) {
                eval_cdr(vec, env)
            } else if is_symbol("cons", &vec[0]) {
                eval_cons(vec, env)
            } else if is_symbol("cond", &vec[0]) {
                eval_cond(vec, env)
            } else {

                let num_args = vec.len() - 1;
                let res = prepare_lambda(vec, env);

                if res.is_err() {
                    return Err( res.unwrap_err() );
                }

                let (mut lambda, mut bindings, args) = res.unwrap();


                let mut lambda_iter = lambda.move_iter();
                lambda_iter.next(); // discard "lambda" symbol, not needed
                let params: ~[Expr] = lambda_iter.next().unwrap().unwrap_branch();
                let lambda_body: Expr = lambda_iter.next().unwrap();

                if params.len() != num_args {
                    return Err(~"mismatch between number of procedure args and number of args called with.");
                }

                let new_binds = populate_bindings(args, params, env, bindings);
                if new_binds.is_err() {
                    return Err( new_binds.unwrap_err() );
                }

                let new_env = Environment { parent: Some(env), 
                                            bindings: new_binds.unwrap() };

                debug!("\n :: arguments have been passed into environment, evaling lambda body\n");
                eval(lambda_body, &new_env)

            }
        }
    }
}

fn eval_atom(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {
    if vec.len() != 2 {
        Err(~"`atom` expects exactly one argument.")
    } else {
        result_bind(eval(vec[1], env), 
                    |val: Expr| -> Result<Expr, ~str> {
                        if val.is_atom() || val.is_empty_list() {
                            Ok( Atom(~"t") )
                        } else {
                            Ok( List(~[]) )
                        }
                    })
    }
}


fn eval_eq(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {

    if vec.len() != 3 {
        Err(~"`eq` expects exactly two arguments.")
    } else {
        result_fmap2(eval(vec[1].clone(), env), 
                     eval(vec[2], env), 
             |val1: Expr, val2: Expr| -> Result<Expr, ~str> {
                if (val1.is_empty_list() && val2.is_empty_list())
                   || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
                    Ok( Atom(~"t") )
                } else {
                    Ok( List(~[]) )
                }
            })
    }
}


fn eval_car(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {

    if vec.len() != 2 {
        Err(~"`car` expects exactly one argument.")
    } else {
        result_bind(eval(vec[1], env), 
                    |val: Expr| -> Result<Expr, ~str> {
                        if val.is_list() && !val.is_empty_list() {
                            let list = val.unwrap_branch();
                            Ok( list[0] )
                        } else {
                            Err(~"`car`'s argument must be a non-empty list")
                        }
                    })
    }
}

fn eval_cdr(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {

    if vec.len() != 2 {
        Err(~"`cdr` expects exactly one argument.")
    } else {
        result_bind(eval(vec[1], env), 
                    |val: Expr| -> Result<Expr, ~str> {
                        if val.is_list() && !val.is_empty_list() {
                            let mut list = val.unwrap_branch();
                            list.shift();
                            Ok( List(list) )
                        } else {
                            Err(~"`cdr`'s argument must be a non-empty list")
                        }
                })
    }
}

fn eval_cons(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {

    if vec.len() != 3 {
        Err(~"`cons` expects exactly two arguments.")
    } else {
        result_fmap2(eval(vec[1].clone(), env), 
                     eval(vec[2], env), 
             |val1: Expr, val2: Expr| -> Result<Expr, ~str> {
                if val2.is_list() {
                    let mut list = val2.unwrap_branch();
                    list.unshift(val1);
                    Ok( List(list) )
                } else {
                    Err(~"`cons`'s second argument must be a list")
                }
            })
    }
}

fn eval_cond(vec: ~[Expr], env: &Environment) -> Result<Expr, ~str> {
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

                if val.eq( &Atom(~"t") ) {
                    return eval(list[1], env);
                }
            }
        }

        i += 1;
    }

    Err(~"No branch of `cond` evaluated to true. Don't think this is an error, though. Need to decide how to handle.")
}


fn is_func_literal(expr: &Expr) -> bool {
    is_lambda_literal(expr) || is_label_literal(expr)
}

fn is_lambda_literal(expr: &Expr) -> bool {
    if !expr.is_list() {
        return false;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_list() 
       || !is_symbol("lambda", &vec[0]) {
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

fn is_label_literal(expr: &Expr) -> bool {
    if !expr.is_list() {
        return false;
    }

    let vec = expr.get_ref_branch();

    if vec.len() != 3 
       || !vec[1].is_atom() 
       || !is_symbol("label", &vec[0]) 
       || !is_lambda_literal(&vec[2]) {
        return false;
    }

    true
}

fn is_symbol(op: &str, expr: &Expr) -> bool {
    if expr.is_atom() {
        let expr_op = expr.get_ref_leaf();
        op.equiv(expr_op)
    } else {
        false
    }
}

fn prepare_lambda(vec: ~[Expr], env: &Environment)
    -> Result<(~[Expr], HashMap<~str, Expr>, MoveItems<Expr>), ~str> {

    let mut vec_iter = vec.move_iter();
    let mut op_expr = vec_iter.next().unwrap();

    // There are two kinds of function calls: lambda calls and label
    // calls. Labels are just lambdas that can cal themselves.
    //
    // We need to distinguish between literal function calls, which
    // take the form:
    //     ((lambda params body) expr1 ... exprn)
    // or 
    //     (label sym ((lambda params body)) expr1 ... exprn)
    //
    // and non-literals, which take the form (expr0 expr1 ... exprn),
    // where expr1 is an expression that evaluates to a function
    // literal. The reason we need to make this distinction is that
    // label and lambda expressions do not evaluate to anything, so 
    // if the operator expression is such a literal we must not eval
    // it. However, if it is not such a literal, we need to eval it
    // to see whether it evaluates to a function literal.

    if !is_func_literal(&op_expr) {
        let res = eval(op_expr, env);

        if res.is_err() {
            return Err( res.unwrap_err() );
        } else {
            op_expr = res.unwrap();

            if !is_func_literal(&op_expr) {
                return Err(~"Unrecognized expression.");
            }
        }
    }

    // If we've made it here, op_expr is either a label or lambda literal
    let lambda: ~[Expr];
    let mut bindings = HashMap::<~str, Expr>::new();

    if is_label_literal(&op_expr) {
        let label_expr = op_expr.clone();

        let label = op_expr.unwrap_branch();
        let mut label_iter = label.move_iter();
        label_iter.next(); //discard "label" symbol

        let func_sym = label_iter.next().unwrap().unwrap_leaf();

        bindings.insert(func_sym, label_expr);

        lambda = label_iter.next().unwrap().unwrap_branch();
    } else {
        lambda = op_expr.unwrap_branch();
    }

    Ok( (lambda, bindings, vec_iter) )
}


/// takes a vector of expressions and a vector of Atoms, evals each expression
/// and inserts it into a provided hashMap (with the Atom as the key)
fn populate_bindings(mut args: MoveItems<Expr>, params: ~[Expr],
     env: &Environment, mut bindings: HashMap<~str, Expr>) 
    -> Result<HashMap<~str, Expr>, ~str> {

    let mut param_iter = params.move_iter();

    debug!("\n :: iterating through args now and passing them into bindings\n");
    for arg in args {
        debug!("  -- {}", arg);
        let res = eval(arg, env);

        if res.is_err() {
            return Err( res.unwrap_err() );
        } else {

            let next_param: Expr  = param_iter.next().unwrap();
            bindings.insert(next_param.unwrap_leaf(),
                            res.unwrap());
        }

    }

    Ok( bindings )
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
