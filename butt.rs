use std::fmt;

fn main() {
    let x = eval( Leaf(~"5") );
    println!("{}", x);

    let y: ~[int] = ~[1, 5, 19, -21];

    for &x in y.iter() {
        println!("hahahah {}", x);
    }

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
            Ok(BSymbol(x))
        },
        _       => Err(~"not implemented")
    }
}
