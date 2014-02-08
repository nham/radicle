#[crate_id = "radicle"];

//! A lisp interpreter.

pub use std::hashmap::HashMap;
pub use std::vec::MoveItems;
use std::str;
use std::os;

use tree::Tree;
pub use tree::Nil;
pub use Atom = tree::Leaf;
pub use List = tree::Branch;

use eval::eval;
use read::read;

pub mod tree;
pub mod eval;
pub mod read;
mod test;

fn main() {
    let args = os::args();
    if args.len() == 1 {
        repl();
    } else if args.len() > 2 {
        println!("radicle: Only one argument allowed.");
    } else {
        interpret_file(args[1]);
    }
}

pub fn interpret_file(fname: ~str) {
    use std::io::File;
    let path = Path::new(fname.clone());

    if path.is_file() {
        let mut hw_file = File::open(&path);
        let contents = hw_file.read_to_end();
        if contents.is_err() {
            println!("{}", contents.unwrap_err());
        } else {
            let data = str::from_utf8_owned(contents.unwrap());
            read_eval(data.unwrap(), &Env::new());
        }
    } else {
        println!("radicle: can't open file {}", fname);
    }
}

pub fn repl() {
    use std::io::BufferedReader;
    use std::io::stdin;
    use std::io::stdio;

    let env = Env::new();
    let mut stdin = BufferedReader::new(stdin());
    print!("repl> ");
    stdio::flush();
    for line in stdin.lines() {
        read_eval(line, &env);
        print!("repl> ");
        stdio::flush();
    }

}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: &Env) {
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


/// The representation of Lisp expressions
pub type Expr = Tree<~str>;
pub type Exprs = ~[Expr];


pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    bindings: HashMap<~str, Expr>,
}

impl<'a> Env<'a> {
    fn new() -> Env<'a> {
        Env { parent: None, bindings: HashMap::new() }
    }

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
