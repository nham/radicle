use std::char::is_whitespace;
use std::vec::MoveItems;
use std::iter::Peekable;

fn main() {

    print_tokens(tokenize("559294)"));
    println!("-----------");
    print_tokens(tokenize("( 559294 x 79% ()  )"));
}

fn print_tokens(mut v: TokenStream) {
    for e in v {
        println!(".{}.", e);
    }
}

type Expression = Tree<~str>;

enum Tree<T> {
    Node(T, ~[Tree<T>])
}

type TokenStream = Peekable<~str, MoveItems<~str>>;


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

fn read_from(mut v: TokenStream) -> (TokenStream, Result<Expression, &str>) {
    let tok = v.next();
    match tok {
        None        => (v, Err("Unexpected end of token stream")),
        Some(s) =>
            if "(".equiv(&s) {
                let mut ch = ~[];

                loop {
                    match v.peek() {
                        Some(x) if ")".equiv(x) => { break; },
                        _                   => {
                            let (w, res) = read_from(v);
                            v = w;

                            match res {
                                Err(e) => return (v, Err(e)),
                                Ok(expr) => { ch.push(expr); }
                            }
                        }
                    }
                }

                (v, Ok(Node(s, ch)))

            } else if ")".equiv(&s) {
                (v, Err("Unexpected ')'"))
            } else {
                (v, Ok(Node(s, ~[])))
            }
    }
}
