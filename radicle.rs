use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

use tree::Tree;
use Atom = tree::Leaf;
use List = tree::Branch;

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
    read_eval("(atom ())");
    read_eval("(atom (my little pony))");
    read_eval("(atom (quote x))");

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
fn eval(expr: Expression) -> Result<Expression, &str> {
    match expr {
        Atom(_) => Ok(expr),
        List([]) => Err("No procedure to call. TODO: a better error message?"),
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
                        Err("`quote` expects exactly one argument.")
                    } else {
                        Ok( vec[1] )
                    }
                },
                Op_Atom => {
                    if vec.len() != 2 {
                        Err("`atom` expects exactly one argument.")
                    } else {
                        if vec[1].is_atom() || vec[1].eq(&empty) {
                            Ok( t )
                        } else {
                            Ok( empty )
                        }
                    }
                },
                Op_Eq => {
                    if vec.len() != 3 {
                        Err("`eq` expects exactly two arguments.")
                    } else {
                        if (vec[1].eq(&empty) && vec[2].eq(&empty))
                           || (vec[1].is_atom() && vec[2].is_atom() && vec[1].eq(&vec[2])) {
                            Ok( t )
                        } else {
                            Ok( empty )
                        }
                    }
                },
                Op_Car => {
                    if vec.len() != 2 {
                        Err("`car` expects exactly one argument.")
                    } else {
                        if vec[1].is_list() && !vec[1].eq(&empty) {
                            let list = vec[1].unwrap_branch();
                            Ok( list[0] )
                        } else {
                            Err("`car`'s argument must be a non-empty list")
                        }
                    }
                },
                Op_Cdr => {
                    Err("not implemented")
                },
                Op_Cons => {
                    Err("not implemented")
                },
                Op_Cond => {
                    Err("not implemented")
                },
                Op_Proc => {
                    Err("not implemented")
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


mod tree {
    use std::fmt::{Default, Formatter};

    #[deriving(Eq, Clone)]
    pub enum Tree<T> {
        Leaf(T),
        Branch(~[Tree<T>])
    }

    impl<T> Tree<T> {
        pub fn is_leaf(&self) -> bool {
            match *self {
                Leaf(_) => true,
                _       => false
            }
        }

        pub fn is_branch(&self) -> bool {
            !self.is_leaf()
        }

        pub fn get_ref_leaf<'a>(&'a self) -> &'a T {
            match *self {
                Leaf(ref val) => val,
                _         => fail!("called Tree<T>::unwrap_leaf() on Branch()"),
            }
        }

        pub fn unwrap_branch(self) -> ~[Tree<T>] {
            match self {
                Branch(val) => val,
                _         => fail!("called Tree<T>::unwrap_branch() on Branch()"),
            }
        }

    }

    impl<T: Default> Default for Tree<T> {
        fn fmt(v: &Tree<T>, f: &mut Formatter) {
            match *v {
                Branch(ref vec) => write!(f.buf, "Branch{}", *vec),
                Leaf(ref val) => write!(f.buf, "Leaf({})", *val)
            }
        }
    }

    impl<T: Default> Default for ~[Tree<T>] {
        fn fmt(v: &~[Tree<T>], f: &mut Formatter) {
            write!(f.buf, "[");

            for x in v.iter() {
                write!(f.buf, " {}", *x);

            }

            write!(f.buf, " ]");
        }
    }

}
