use std::fmt;
use std::from_str::from_str;

fn main() {
    let x = eval( Leaf(~"5") );
    println!("{}", x);

    println!("uhhhhhh {}", BVector(~[~BNil, ~BChar('z'), ~BBoolean(true)]));

}

enum Expression {
    Leaf(~str),
    Node(~[~Expression])
}

// inspired by 3.4, disjointness of types
// see also 6.3 for discussion of the empty list, called BNil here
enum BValue {
    BBoolean(bool),
    BSymbol(~str),
    BChar(char),
    BNumber(f64),
    BString(~str),
    BVector(~[~BValue]),
    //BProcedure
    BCons(~BValue, ~BValue),
    BNil
}

impl fmt::Default for BValue {
    fn fmt(v: &BValue, f: &mut fmt::Formatter) {
        match *v {
            BBoolean(b)    => write!(f.buf, "BBoolean({})", b),
            BSymbol(ref s) => write!(f.buf, "BSymbol({})", *s),
            BChar(c)       => write!(f.buf, "BChar({})", c),
            BNumber(n)     => write!(f.buf, "BNumber({})", n),
            BString(ref s) => write!(f.buf, "BNumber({})", *s),
            BVector(ref v) => write!(f.buf, "BVector({})", *v),
            BCons(ref v1, ref v2) => write!(f.buf, "BCons({}, {})", **v1, **v2),
            BNil => write!(f.buf, "()")
        }
    }
}

impl fmt::Default for ~[~BValue] {
    fn fmt(v: &~[~BValue], f: &mut fmt::Formatter) {
        write!(f.buf, "[");

        for &ref x in v.iter() {
            write!(f.buf, " {}", **x);

        }

        write!(f.buf, " ]");
    }
}

fn eval(expr: Expression) -> Result<BValue, ~str> {
    match expr {
        Leaf(x) => {
            if is_num(x) {
                //Ok(BNumber(x))
                match from_str::<f64>(x) {
                    Some(x) => Ok( BNumber(x) ),
                    None    => Err(~"I thought it was a number, but it's not?")
                }
            } else {
                Ok( BSymbol(x) )
            }
        },
        _       => Err(~"not implemented")
    }
}


fn is_num(s: &str) -> bool {
    for c in s.chars() {
        if c < '0' || c > '9' {
            return false;
        }
    }

    true
}

#[test]
fn test_is_num() {
    assert!(is_num("9"));
    assert!(is_num("0"));
    assert!(is_num("458915"));
    assert!(is_num("0000009999"));
}
