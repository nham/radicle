use std::char::is_whitespace;
use std::iter::Peekable;

use super::{Expr, Exprs, MoveItems, Atom, List};

/// Intermediate representation after tokenization and before it gets read into
/// an expression.
pub type TokenStream = Peekable<~str, MoveItems<~str>>;


/// Tries to read a string of symbols into a list of expressions
pub fn read(s: &str) -> Result<Exprs, ~str> {
    let mut stream = tokenize(s);

    let mut res: Exprs = ~[];

    while !stream.is_empty() {
        let x = read_from(&mut stream);
        if x.is_err() {
            return Err( x.unwrap_err() );
        } else {
            res.push( x.unwrap() );
        }
    }

    Ok( res )
}


/// Turns a string into a stream of tokens. Currently assumes that tokens
/// do not have spaces or parens/brackets/braces in them.
pub fn tokenize(s: &str) -> TokenStream {
    let mut s1 = s.replace("(", " ( ").replace(")", " ) ");
    s1 = s1.replace("[", " [ ").replace("]", " ] ");
    s1 = s1.replace("{", " { ").replace("}", " } ");
    s1 = s1.replace("'", " ' ");

    let x: ~[&str] = s1.split(|c: char| is_whitespace(c)).collect();

    let mut ret: ~[~str] = ~[];
    for &e in x.iter() {
        if e != "" {
            ret.push(e.to_owned());
        }
    }

    ret.move_iter().peekable()
}

/// Attempts to read an entire expression from the token stream. Detects
/// mismatched parentheses. Also expands ' <expr> into (quote <expr)
pub fn read_from(v: &mut TokenStream) -> Result<Expr, ~str> {
    fn is_beginning_list_sep(s: &~str) -> bool {
        "(".equiv(s) || "[".equiv(s) || "{".equiv(s)
    }

    fn is_ending_list_sep(s: &~str) -> bool {
        ")".equiv(s) || "]".equiv(s) || "}".equiv(s)
    }

    let tok = v.next();
    match tok {
        None        => Err(~"Unexpected end of token stream"),
        Some(s) =>
            if is_beginning_list_sep(&s) {
                let mut ch = ~[];

                loop {
                    {
                        let x = v.peek();
                        if x.is_some() && is_ending_list_sep( x.unwrap()) {
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

            } else if is_ending_list_sep(&s) {
                Err(format!("Unexpected '{}'", s))
            } else if "'".equiv(&s) {
                match read_from(v) {
                    Err(e) => Err(e),
                    Ok(expr) => Ok( List(~[Atom(~"quote"), expr]) ),
                }
            } else {
                Ok( Atom(s) )
            }
    }
}
