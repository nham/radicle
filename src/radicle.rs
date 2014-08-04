//! A lisp interpreter.

#![crate_name = "radicle"]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate debug;

pub use std::collections::HashMap;
pub use std::vec::MoveItems;
use std::str;
use std::os;

pub use tree::{Tree, Nil};
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
        interpret_file(args[1].clone());
    }
}

pub fn interpret_file(fname: String) {
    use std::io::File;
    let path = Path::new(fname.clone());

    if path.is_file() {
        let mut hw_file = File::open(&path);
        match hw_file.read_to_string() {
            Err(e) => println!("{}", e),
            Ok(s) => {
                read_eval(s, &mut Env::new());
            }
        }
    } else {
        println!("radicle: can't open file {}", fname);
    }
}

pub fn repl() {
    use std::io::BufferedReader;
    use std::io::stdin;
    use std::io::stdio;

    let mut env = Env::new();
    let mut stdin = BufferedReader::new(stdin());
    print!("repl> ");
    stdio::flush();
    for line in stdin.lines() {
        read_eval(line.unwrap(), &mut env);

        print!("repl> ");
        stdio::flush();
    }

}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: String, env: &mut Env) {
    let parsed = read(s.as_slice());
    if parsed.is_ok() {
        for expr in parsed.unwrap().move_iter() {
            match eval(env, expr) {
                Ok(Nil) => {},
                Ok(expr) => expr.print(),
                Err(ref x) => println!("\nError: {}", *x),
            }
        }

    } else {
        println!("\nParse error: {}", parsed.err().unwrap());
    }
}


/// The representation of Lisp expressions
pub type Expr = Tree<String>;
pub type Exprs = Vec<Expr>;


#[deriving(Clone)]
pub struct Env {
    bindings: HashMap<String, Expr>,
}

impl Env {
    fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    fn find_copy(&self, key: &String) -> Option<Expr> {
        self.bindings.find_copy(key)
    }
}

/// Wrapping the standard Tree methods for aesthetic reasons, I guess
impl ::tree::Tree<String> {
    fn is_atom(&self) -> bool {
        self.is_leaf()
    }

    fn is_list(&self) -> bool {
        self.is_branch()
    }

    fn is_empty_list(&self) -> bool {
        self.eq(&List(vec!()))
    }
}
