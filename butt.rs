use std::fmt::{Default, Formatter};
use std::from_str::from_str;
use std::str::{eq_slice, eq};
use std::option::{Option};
use std::hashmap::HashMap;
use std::clone::Clone;
use std::cmp::Eq;

fn main() {
    let mut env: Environment = HashMap::new();
    env.insert(~"x", BNumber(17f64));

    let mut inp = "-3.14159";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "x";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "'z'";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "\"friends\"";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "()";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "(+ 1 1)";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "(+ 1 2 3 4)";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "(+ 3.14159 -3 x)";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }

    inp = "(+)";
    match parse(inp) {
        Ok(expr) => println!("input: {}, eval: {}", inp, eval(&expr, &env)),
        Err(s)   => println!("{}", s)
    }


    let a = parse("(5");
    println!("{}", a);

    let b = parse("(5 7)");
    println!("{}", b);

    println!("{}", BVector(~[BPair( ~Cons(BNumber(3.14159), ~Nil) ), 
                                     BChar('z'), BBoolean(true)]));

}

#[deriving(Clone, Eq)]
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
#[deriving(Clone)]
enum BValue {
    BBoolean(bool),
    BSymbol(~str),
    BChar(char),
    BNumber(f64),
    BString(~str),
    BVector(~[BValue]),
    BPair(~List<BValue>),
    BProcedure(BProcedure),
}

#[deriving(Clone)]
struct BProcedure {
    args: ~[~str],
    body: Expression,
    env: Environment
}

#[deriving(Clone)]
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
            BPair(ref p) => write!(f.buf, "BPair({})", **p),
            BProcedure(_) => write!(f.buf, "BProcedure()")
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
    if eq_slice(s, "()") {
        let x: ~[Expression] = ~[];
        return Ok( Node(x) );
    }

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

fn eval(expr: &Expression, env: &Environment) -> Result<BValue, ~str> {
    match *expr {
        Leaf(ref x) => {
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
        Node([]) => Err(~"TODO: error message for ()"),
        Node([ref f, ..rest]) => {
            //if f is builtin, we do something special.
            if f.eq( &Leaf(~"+") ) {
                eval_plus(rest, env)
            } else {
                eval(f, env)
            }
        }
    }
}


fn eval_bool(s: &~str) -> Option<bool> {
    if eq(s, &~"#t") {
        Some(true)
    } else if eq(s, &~"#f") {
        Some(false)
    } else {
        None
    }
}

fn eval_num(s: &~str) -> Option<f64> {
    from_str::<f64>(*s)
}

fn eval_char(s: &~str) -> Option<char> {
    let x: ~[char] = s.chars().collect();
    if x.len() == 3 && x[0] == '\'' && x[2] == '\'' {
        Some(x[1])
    } else {
        None
    }
}

fn eval_string(s: &~str) -> Option<~str> {
    let len = s.char_len();
    if len > 1 && s.char_at(0) == '"' && s.char_at(len - 1) == '"' {
        Some(s.slice(1, len - 1).to_owned())
    } else {
        None
    }
    
}

fn eval_symbol(s: &~str, env: &Environment) -> Option<BValue> {
    //println!("{}", env);
    // env.find
    let key = &s.to_owned();
    match env.find_copy(key) {
        Some(x) => Some(x),
        None     => Some(BSymbol(s.to_owned()))
    }
}


fn eval_plus(args: &[Expression], env: &Environment) -> Result<BValue, ~str> {
    let mut sum = 0.0;

    for arg in args.iter() {
        match eval(arg, env) {
            Err(x) => {
                return Err(x);
            },

            Ok(ref x) =>
                match *x {
                    BNumber(num) => {
                        sum += num;
                    },
                    _ => {
                        return Err(~"Argument is not a number");
                    }
                }
        } // end match
    } // end for

    Ok( BNumber(sum) )

}
