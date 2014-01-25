use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

use std::hashmap::HashMap;
use std::fmt::{Default, Formatter};
use std::from_str::from_str;

fn main() {
    let env = Environment{ parent: None, env: HashMap::new() };

    read_eval("5678)", &env);
    println!("-----------");
    read_eval("wonky", &env);
    println!("-----------");
    read_eval("( 559294 x 79% ()  )", &env);
    println!("-----------");
    read_eval("(gub (middle) end)", &env);
    println!("-----------");
    read_eval("(one 2)", &env);

}

fn read_eval(s: &str, env: &Environment) {
    println!("input: {}", s);
    let parsed = read(s);
    if parsed.is_ok() {
        match eval(parsed.unwrap(), env) {
            Ok(x) => { println!("evaled: {}", x); },
            Err(x) => { println!("Eval error: {}", x); }
        }
    } else {
        println!("Parse error: {}", parsed.unwrap_err());
    }
}


type TokenStream = Peekable<~str, MoveItems<~str>>;

enum Atom {
    Symbol(~str),
    Number(f64)
}

impl Default for Atom {
    fn fmt(a: &Atom, f: &mut Formatter) {
        match *a {
            Symbol(ref sym) => write!(f.buf, "{}", *sym),
            Number(num) => write!(f.buf, "{}", num)
        }

    }
}


enum Tree<T> {
    Leaf(T),
    Branch(~[Tree<T>])
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


type Expression = Tree<Atom>;

struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    env: HashMap<~str, Expression>,
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
                Ok( Branch(ch) )

            } else if ")".equiv(&s) {
                Err("Unexpected ')'")
            } else {
                let x = from_str::<f64>(s);

                if x.is_some() {
                    Ok( Leaf(Number(x.unwrap())) )
                } else {
                    Ok( Leaf(Symbol(s)) )
                }
            }
    }
}


// given that to do procedure calls, we want to evaluate all of the elements
// of a list then pass the code directly to the procedure, quote *must* return
// a "typed" representation.
//   (cons 1 (quote (4 + 9 5))) is valid
fn eval(expr: Expression, env: &Environment) -> Result<Expression, &str> {
    match expr {
        Leaf(_) => Ok(expr),
        Branch(ref b) => Err("not implemented")
    }
}

