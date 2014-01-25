use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

use std::fmt::{Default, Formatter};
use std::from_str::from_str;

fn main() {

    let mut parsed = read("559294)");
    match parsed {
        Ok(x) => { println!("{}", x); },
        Err(x) => { println!("{}", x); }
    }

    println!("-----------");
    parsed = read("( 559294 x 79% ()  )");
    match parsed {
        Ok(x) => { println!("{}", x); },
        Err(x) => { println!("{}", x); }
    }

    println!("{}", Branch(~[Leaf(~"gub"), 
                            Branch(~[Leaf(~"middle")]),
                            Leaf(~"end")]));

    parsed = read("(gub (middle) end)");
    match parsed {
        Ok(x) => { println!("{}", x); },
        Err(x) => { println!("{}", x); }
    }

    parsed = read("(one 2");
    match parsed {
        Ok(x) => { println!("{}", x); },
        Err(x) => { println!("{}", x); }
    }

}

type Expression = Tree<Atom>;

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


fn read(s: &str) -> Result<Expression, &str> {
    let mut stream = tokenize(s);
    let x = read_from(&mut stream);

    // eventually this will be stream.is_empty(), but theres a bug rust!
    if stream.peek().is_some() {
        return Err("Tokens left over, so parse was unsuccessful.");
    }

    x
}


// only works with expressions separated
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
