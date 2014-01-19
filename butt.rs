use std::fmt::{Default, Formatter};
use std::from_str::from_str;
use std::str::{eq_slice};
use std::option::{Option};

fn main() {
    let x = parse("-3.14159");
    println!("{}", x);

    let y = parse("'z'");
    println!("{}", y);

    let z = parse("\"friends\"");
    println!("{}", z);

    let a = tokenize("(5");
    println!("{}", a);

    let b = tokenize("(5 7)");
    println!("{}", b);

    println!("uhhhhhh {}", BVector(~[BPair( ~Cons(BNumber(3.14159), ~Nil) ), 
                                     BChar('z'), BBoolean(true)]));

}

enum Expression {
    Leaf(~str),
    Node(~[Expression])
}

impl Default for ~[Expression] {
    fn fmt(v: &~[Expression], f: &mut Formatter) {
        write!(f.buf, "[");

        for x in v.iter() {
            write!(f.buf, " {}", *x);

        }

        write!(f.buf, " ]");
    }
}

impl Default for Expression {
    fn fmt(v: &Expression, f: &mut Formatter) {
        match *v {
            Node(ref vec) => write!(f.buf, "Node({})", *vec),
            Leaf(ref val) => write!(f.buf, "Leaf({})", *val)
        }
    }
}

// inspired by 3.4, disjointness of types
// see also 6.3 for discussion of the empty list, called BNil here
enum BValue {
    BBoolean(bool),
    BSymbol(~str),
    BChar(char),
    BNumber(f64),
    BString(~str),
    BVector(~[BValue]),
    //BProcedure
    BPair(~List<BValue>),
}

enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

impl<T: Default> Default for List<T> {
    fn fmt(v: &List<T>, f: &mut Formatter) {
        match *v {
            Cons(ref v, ref l) => write!(f.buf, "({} : {})", *v, **l),
            Nil => write!(f.buf, "()")
        }
    }
}

impl Default for BValue {
    fn fmt(v: &BValue, f: &mut Formatter) {
        match *v {
            BBoolean(b)    => write!(f.buf, "BBoolean({})", b),
            BSymbol(ref s) => write!(f.buf, "BSymbol({})", *s),
            BChar(c)       => write!(f.buf, "BChar({})", c),
            BNumber(n)     => write!(f.buf, "BNumber({})", n),
            BString(ref s) => write!(f.buf, "BString({})", *s),
            BVector(ref v) => write!(f.buf, "BVector({})", *v),
            BPair(ref p) => write!(f.buf, "BPair({})", **p)
        }
    }
}

impl Default for ~[BValue] {
    fn fmt(v: &~[BValue], f: &mut Formatter) {
        write!(f.buf, "[");

        for x in v.iter() {
            write!(f.buf, " {}", *x);

        }

        write!(f.buf, " ]");
    }
}

fn tokenize(s: &str) -> Result<Expression, ~str> {
    let s1 = s.replace("(", "( ").replace(")", " )");
    let tokens: ~[&str] = s1.split(' ').collect();
    if !eq_slice(tokens[0], "(") {
        Ok( Leaf(tokens[0].to_owned()) )
    } else {
        // This doesnt tokenize nested lists yet
        if !eq_slice(tokens[tokens.len() - 1], ")") {
            return Err(~"Mismatched parentheses");
        }

        let mut i = 1;
        let mut toks: ~[Expression] = ~[];

        while !eq_slice(tokens[i], ")") && i < tokens.len() {
            toks.push( Leaf(tokens[i].to_owned()) );
            i += 1;
        }

        return Ok( Node(toks) );
    }
}

fn parse(s: &str) -> Result<BValue, ~str> {
    match tokenize(s) {
        Err(s) => Err(s),
        Ok(expr) =>
            match expr {
                Leaf(x) => {
                    match parse_bool(x) {
                        Some(b) => Ok( BBoolean(b) ),
                        None    => 
                    match parse_char(x) {
                        Some(c) => Ok( BChar(c) ),
                        None    =>
                    match parse_num(x) {
                        Some(n) => Ok( BNumber(n) ),
                        None    => 
                    match parse_string(x) {
                        Some(s) => Ok( BString(s) ),
                        None    => Ok( BSymbol(x) )
                    }
                    }
                    }
                    }
                },
                Node(_) => Err(~"not implemented")
            }
    }
}

fn eval(val: BValue) -> ~str {
    fail!("Unimplemented");
}


fn parse_bool(s: &str) -> Option<bool> {
    if eq_slice(s, "#t") {
        Some(true)
    } else if eq_slice(s, "#f") {
        Some(false)
    } else {
        None
    }
}

fn parse_num(s: &str) -> Option<f64> {
    from_str::<f64>(s)
}

fn parse_char(s: &str) -> Option<char> {
    let x: ~[char] = s.chars().collect();
    if x.len() == 3 && x[0] == '\'' && x[2] == '\'' {
        Some(x[1])
    } else {
        None
    }
}

fn parse_string(s: &str) -> Option<~str> {
    let len = s.char_len();
    if len > 1 && s.char_at(0) == '"' && s.char_at(len - 1) == '"' {
        Some(s.slice(1, len - 1).to_owned())
    } else {
        None
    }
    
}
