use std::iter::Peekable;

use super::{Expr, MoveItems, Atom, List};

/// Intermediate representation after tokenization and before it gets read into
/// an expression.
pub type TokenStream = Peekable<String, MoveItems<String>>;

/// Tries to read a string of symbols into a list of expressions
pub fn read(s: &str) -> Result<Vec<Expr>, &'static str> {
    let mut stream = tokenize(s);
    let mut res = vec!();

    while !stream.is_empty() {
        match read_from(&mut stream) {
            Err(e) => return Err(e),
            Ok(x) => res.push(x),
        }
    }
    Ok(res)
}

/// Turns a string into a stream of tokens. Currently assumes that tokens
/// do not have spaces or parens/brackets/braces in them.
pub fn tokenize(s: &str) -> TokenStream {
    let mut s1 = s.replace("(", " ( ").replace(")", " ) ");
    s1 = s1.replace("[", " [ ").replace("]", " ] ");
    s1 = s1.replace("{", " { ").replace("}", " } ");
    s1 = s1.replace("'", " ' ");

    let x: Vec<&str> = s1.as_slice().split(|c: char| c.is_whitespace()).collect();

    let mut ret: Vec<String> = vec!();
    for &e in x.iter() {
        if e != "" {
            ret.push(e.to_string());
        }
    }

    ret.into_iter().peekable()
}

/// Attempts to read an entire expression from the token stream. Detects
/// mismatched parentheses. Also expands ' <expr> into (quote <expr)
pub fn read_from(v: &mut TokenStream) -> Result<Expr, &'static str> {
    let tok = v.next();
    match tok {
        None        => Err("Unexpected end of token stream"),
        Some(s) =>
            if is_beginning_list_sep(&s) {
                let mut ch = vec!();
                loop {
                    if is_end(v) { break; }
                    match read_from(v) {
                        Err(e) => { return Err(e); },
                        Ok(expr) => { ch.push(expr); }
                    }
                }

                v.next();
                Ok( List(ch) )

            } else if is_ending_list_sep(&s) {
                Err("Unexpected list end token")
            } else if "'".equiv(&s) {
                match read_from(v) {
                    Err(e) => Err(e),
                    Ok(expr) => Ok( List( vec!(Atom("quote".to_string()), expr)) ),
                }
            } else {
                Ok( Atom(s) )
            }
    }
}

fn is_end(v: &mut TokenStream) -> bool {
    let x = v.peek();
    x.is_some() && is_ending_list_sep(x.unwrap())
}

fn is_beginning_list_sep(s: &String) -> bool {
    "(".equiv(s) || "[".equiv(s) || "{".equiv(s)
}

fn is_ending_list_sep(s: &String) -> bool {
    ")".equiv(s) || "]".equiv(s) || "}".equiv(s)
}
