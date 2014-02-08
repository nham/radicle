#[crate_id = "radicle"];

//! A lisp interpreter.

use std::char::is_whitespace;
pub use std::hashmap::HashMap;
pub use std::vec::MoveItems;
use std::iter::Peekable;
use std::str;
use std::io::File;
use std::os;

use tree::Tree;
pub use tree::Nil;
pub use Atom = tree::Leaf;
pub use List = tree::Branch;

use repl::do_repl;
use eval::eval;

pub mod tree;
pub mod repl;
pub mod eval;
mod test;

fn main() {
    let globenv = Environment { parent: None, bindings: HashMap::new() };

    let args = os::args();
    if args.len() == 1 {
        do_repl();
        return;
    } else if args.len() > 2 {
        println!("radicle: Only one argument allowed.");
        return;
    }

    if !"--test".equiv(&args[1]) {
        let fname = args[1].clone();
        let path = Path::new(args[1]);
        if path.is_file() {
            let mut hw_file = File::open(&path);
            let contents = hw_file.read_to_end();
            if contents.is_err() {
                println!("{}", contents.unwrap_err());
            } else {
                let data = str::from_utf8_owned(contents.unwrap());
                read_eval(data.unwrap(), &globenv);
            }
            return;
        } else {
            println!("radicle: can't open file {}", fname);
            return;
        }
    }

    /*
    read_eval("((lambda (x) (cons x (quote (ab cd)))) (quote CONSME))", &globenv);
    read_eval("((lambda (x y z) (cons y (cons z (cons x (quote (batman)))))) (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);
    read_eval("((lambduh (x) (cons x (quote ()))) (quote CONSME))", &globenv);
    read_eval("(((lambda (x) 
           (lambda (y) (cons x 
                             (cons y 
                                   (quote ()))))) 
   (quote 5)) (quote 6))", &globenv);
    read_eval(
"((label ZABBA (lambda (x) (cons x (quote (ab cd)))))
  (quote CONSME))", &globenv);

    read_eval(
"((label ZABBA (lambda (x y z) (cons y (cons z (cons x (quote (batman)))))))
  (quote CONSME) (quote santa) (car (quote (10 20 30))))", &globenv);


 */
    read_eval("(quote x) (quote y) (quote z)", &globenv);
}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: &Environment) {
    let parsed = read(s);
    if parsed.is_ok() {
        println!("Parsed: {}", parsed);

        for expr in parsed.unwrap().move_iter() {
            match eval(expr, env) {
                Ok(x) => { println!("\nEvaled: {}", x); },
                Err(x) => { println!("\nEval error: {}", x); }
            }
        }
    } else {
        println!("\nParse error: {}", parsed.unwrap_err());
    }
}


/// Intermediate representation after tokenization and before it gets read into
/// and expression.
pub type TokenStream = Peekable<~str, MoveItems<~str>>;

/// The representation of Lisp expressions
pub type Expr = Tree<~str>;
pub type Exprs = ~[Expr];


pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    bindings: HashMap<~str, Expr>,
}

impl<'a> Environment<'a> {
    fn find(&'a self, key: &~str) -> Option<&'a Expr> {
        if self.bindings.contains_key(key) {
            self.bindings.find(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find(key)
            } else {
                None
            }
        }
    }

    fn find_copy(&self, key: &~str) -> Option<Expr> {
        if self.bindings.contains_key(key) {
            self.bindings.find_copy(key)
        } else {
            if self.parent.is_some() {
                self.parent.unwrap().find_copy(key)
            } else {
                None
            }
        }
    }
}

/// Wrapping the standard Tree methods for aesthetic reasons, I guess
impl ::tree::Tree<~str> {
    fn is_atom(&self) -> bool {
        self.is_leaf()
    }

    fn is_list(&self) -> bool {
        self.is_branch()
    }

    fn is_empty_list(&self) -> bool {
        self.eq(&List(~[]))
    }
}


/// Reads a string of symbols into an expression (or possibly an error message)
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
/// do not have spaces or parens in them.
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
/// mismatched parentheses.
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
