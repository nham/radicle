use std::fmt::{Default, Formatter};
use std::from_str::from_str;
use std::str::{eq_slice};
use std::option::{Option};
use std::hashmap::HashMap;

fn main() {
    let mut env: Environment = HashMap::new();

    match parse("-3.14159") {
        Ok(expr) => println!("{}", eval(expr, env)),
        Err(s)   => println!("{}", s)
    }

    let y = parse("'z'");
    println!("{}", y);

    let z = parse("\"friends\"");
    println!("{}", z);

    let a = parse("(5");
    println!("{}", a);

    let b = parse("(5 7)");
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


type Environment = HashMap<~str, BValue>;


fn parse(s: &str) -> Result<Expression, ~str> {
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

fn eval(expr: Expression, env: Environment) -> Result<BValue, ~str> {
    match expr {
        Leaf(x) => {
            match eval_bool(x) {
                Some(b) => Ok( BBoolean(b) ),
                None    => 
            match eval_char(x) {
                Some(c) => Ok( BChar(c) ),
                None    =>
            match eval_num(x) {
                Some(n) => Ok( BNumber(n) ),
                None    => 
            match eval_string(x) {
                Some(s) => Ok( BString(s) ),
                None    => 
            match eval_symbol(x, env) {
               Some(v) => Ok( v ),
               None    => Err(~"Can't figure it out, man")
            }
            }
            }
            }
            }
        },
        Node(_) => Err(~"not implemented")
    }
}


fn eval_bool(s: &str) -> Option<bool> {
    if eq_slice(s, "#t") {
        Some(true)
    } else if eq_slice(s, "#f") {
        Some(false)
    } else {
        None
    }
}

fn eval_num(s: &str) -> Option<f64> {
    from_str::<f64>(s)
}

fn eval_char(s: &str) -> Option<char> {
    let x: ~[char] = s.chars().collect();
    if x.len() == 3 && x[0] == '\'' && x[2] == '\'' {
        Some(x[1])
    } else {
        None
    }
}

fn eval_string(s: &str) -> Option<~str> {
    let len = s.char_len();
    if len > 1 && s.char_at(0) == '"' && s.char_at(len - 1) == '"' {
        Some(s.slice(1, len - 1).to_owned())
    } else {
        None
    }
    
}

fn eval_symbol(s: &str, env: Environment) -> Option<BValue> {
    //println!("{}", env);
    // env.find
    Some(BSymbol(s.to_owned()))
}
