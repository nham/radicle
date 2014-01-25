use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

use tree::Tree;
use Atom = tree::Leaf;
use List = tree::Branch;

mod tree;

fn main() {
/*
    read_eval("5678)");
    println!("-----------");
    read_eval("wonky");
    println!("-----------");
    read_eval("(quote 55)");
    println!("-----------");
    read_eval("( 559294 x 79% ()  )", &env);
    println!("-----------");
    read_eval("(gub (middle) end)", &env);
    println!("-----------");
    read_eval("(one 2)", &env);
    */

    read_eval("(quote x)");
    read_eval("(atom x)");
    read_eval("(atom 1)");
    read_eval("(atom 79.14159)");
    read_eval("(atom (quote ()))");
    read_eval("(atom (my little pony))");
    read_eval("(atom (quote (my little pony)))");
    read_eval("(atom (quote x))");
    read_eval("(atom (atom x))");
    read_eval("(car (quote (10 5 9)))");

}


fn read_eval(s: &str) {
    println!("input: {}", s);
    let parsed = read(s);
    if parsed.is_ok() {
        println!("Parsed: {}", parsed);
        match eval(parsed.unwrap()) {
            Ok(x) => { println!("evaled: {}", x); },
            Err(x) => { println!("Eval error: {}", x); }
        }
    } else {
        println!("Parse error: {}", parsed.unwrap_err());
    }
    println!("-----------");
}


type TokenStream = Peekable<~str, MoveItems<~str>>;

type Expression = Tree<~str>;

impl ::tree::Tree<~str> {
    fn is_atom(&self) -> bool {
        self.is_leaf()
    }

    fn is_list(&self) -> bool {
        self.is_branch()
    }
}


fn read(s: &str) -> Result<Expression, &str> {
    let mut stream = tokenize(s);
    let x = read_from(&mut stream);

    // eventually this will be stream.is_empty(), but theres a bug rust!
    if stream.peek().is_some() {
        return Err("Tokens left over, so parse was unsuccessful.");
    }

    x
}


// assumes that tokens do not have whitespace or parens in them
// this would fail if we add char or string literals
fn tokenize(s: &str) -> TokenStream {
    let s1 = s.replace("(", "( ").replace(")", " )");

    let x: ~[&str] = s1.split(|c: char| is_whitespace(c)).collect();
    
    let mut ret: ~[~str] = ~[];
    for &e in x.iter() {
        if e != "" {
            ret.push(e.to_owned());
        }
    }
    //ret
    ret.move_iter().peekable()
}

fn read_from(v: &mut TokenStream) -> Result<Expression, &str> {
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


// given that to do procedure calls, we want to evaluate all of the elements
// of a list then pass the code directly to the procedure, quote *must* return
// a "typed" representation.
//   (cons 1 (quote (4 + 9 5))) is valid
fn eval(expr: Expression) -> Result<Expression, ~str> {
    match expr {
        Atom(s) => {
            if { 
                let x = &s; 
                ( from_str::<f64>(*x) ).is_some()
            } {
                Ok( Atom(s) )
            } else {
                Err(~"Symbol evaluation is unimplemented")
            }
        },
        List([]) => Err(~"No procedure to call. TODO: a better error message?"),
        List(vec) => {
            let t = Atom(~"t");
            let empty: Expression = List(~[]);

            enum Op {
                Op_Quote,
                Op_Atom,
                Op_Eq,
                Op_Car,
                Op_Cdr,
                Op_Cons,
                Op_Cond,
                Op_Proc,
            }

            let op_type: Op = {
                let p = &vec[0];
                if p.is_atom() {
                    let op = p.get_ref_leaf();
                
                    if "quote".equiv(op) {
                        Op_Quote
                    } else if "atom".equiv(op) {
                        Op_Atom
                    } else if "eq".equiv(op) {
                        Op_Eq
                    } else if "car".equiv(op) {
                        Op_Car
                    } else if "cdr".equiv(op) {
                        Op_Cdr
                    } else if "cons".equiv(op) {
                        Op_Cons
                    } else if "cond".equiv(op) {
                        Op_Cond
                    } else {
                        Op_Proc
                    }
                } else {
                    Op_Proc
                }
            };

            match op_type {
                Op_Quote => {
                    if vec.len() != 2 {
                        Err(~"`quote` expects exactly one argument.")
                    } else {
                        Ok( vec[1] )
                    }
                },
                Op_Atom => {
                    if vec.len() != 2 {
                        Err(~"`atom` expects exactly one argument.")
                    } else {
                        match eval(vec[1]) {
                            Ok(val) =>
                                if val.is_atom() || val.eq(&empty) {
                                    Ok( t )
                                } else {
                                    Ok( empty )
                                },
                            err @ Err(_) => err,
                        }
                    }
                },
                Op_Eq => {
                    if vec.len() != 3 {
                        Err(~"`eq` expects exactly two arguments.")
                    } else {
                        let res1 = eval(vec[1].clone());
                        let res2 = eval(vec[2]);
                        if res1.is_err() {
                            res1
                        } else if res2.is_err() {
                            res2
                        } else {
                            let (val1, val2) = (res1.unwrap(), res2.unwrap());
                            if (val1.eq(&empty) && val2.eq(&empty))
                               || (val1.is_atom() && val2.is_atom() && val1.eq(&val2)) {
                                Ok( t )
                            } else {
                                Ok( empty )
                            }
                        }
                    }
                },
                Op_Car => {
                    if vec.len() != 2 {
                        Err(~"`car` expects exactly one argument.")
                    } else {
                        let res = eval(vec[1]);
                        if res.is_err() {
                            res
                        } else {
                            let val = res.unwrap();
                            if val.is_list() && !val.eq(&empty) {
                                let list = val.unwrap_branch();
                                Ok( list[0] )
                            } else {
                                Err(~"`car`'s argument must be a non-empty list")
                            }
                        }
                    }
                },
                Op_Cdr => {
                    Err(~"not implemented")
                },
                Op_Cons => {
                    Err(~"not implemented")
                },
                Op_Cond => {
                    Err(~"not implemented")
                },
                Op_Proc => {
                    Err(~"not implemented")
                },
            }

            /*
                let mut vals: ~[Expression] = ~[];
                for n in nodes.move_iter() {
                    let val = eval(n, env);

                    if val.is_err() {
                        return val;
                    } else {
                        vals.push(val.unwrap());
                    }
                }
                */
        }
    }
}
