//! A lisp interpreter.

#![crate_id = "radicle"]

#![feature(phase)]
#[phase(syntax, link)] extern crate log;


extern crate collections;

pub use collections::HashMap;
pub use std::vec::MoveItems;
use std::str;
use std::os;

pub use tree::Tree;
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
            let data = str::from_utf8(contents.unwrap().as_slice()).unwrap().to_owned(); // ay yi yi
            read_eval(data, Env::new());
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

        match read_eval(line.unwrap(), env.clone()) {
            Some(new_env) => { env = new_env; },
            None => ()
        }

        print!("repl> ");
        stdio::flush();
    }

}


/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: &str, env: Env) -> Option<Env> {
    let parsed = read(s);
    if parsed.is_ok() {
        let mut expr_it = parsed.unwrap().move_iter();
        let eval_help = |env: Env, expr| {
            let res = eval(env.clone(), expr);
                match res {
                    Ok( (env, Nil) ) => env,
                    Ok( (env, expr) ) => { expr.print(); env },
                    Err(ref x) => { println!("\nError: {}", *x); env }
                }
        };
         Some( expr_it.fold(env, eval_help) )

    } else {
        println!("\nParse error: {}", parsed.err().unwrap());
        None
    }
}


/// The representation of Lisp expressions
pub type Expr = Tree<~str>;
pub type Exprs = Vec<Expr>;


#[deriving(Clone)]
pub struct Env {
    bindings: HashMap<~str, Expr>,
}

impl Env {
    fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    fn find_copy(&self, key: &~str) -> Option<Expr> {
        self.bindings.find_copy(key)
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
        self.eq(&List(vec!()))
    }
}
