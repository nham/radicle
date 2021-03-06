//! A lisp interpreter.

#![crate_name = "radicle"]

pub use std::collections::HashMap;
pub use std::vec::IntoIter;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{stdin, stdout, BufRead, BufReader, Write, Read};

pub use expr::Expression;
pub use expr::Expression::{Nil, Atom, List};

use eval::eval;
use read::read;

pub mod expr;
pub mod eval;
pub mod read;
mod test;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        repl();
    } else if args.len() > 2 {
        println!("radicle: Only one argument allowed.");
    } else {
        args.next();
        interpret_file(args.next().unwrap().clone());
    }
}

pub fn interpret_file(fname: String) {
    let path = Path::new(&fname);

    if path.is_file() {
        let mut hw_file = File::open(&path).expect("Couldn't open file to interpret it.");

        let mut program_text = String::new();
        match hw_file.read_to_string(&mut program_text) {
            Err(e) => println!("{}", e),
            Ok(_) => {
                read_eval(program_text, &mut Env::new());
            }
        }
    } else {
        println!("radicle: can't open file {}", fname);
    }
}

pub fn repl() {
    let mut env = Env::new();
    let mut stdin = BufReader::new(stdin());
    let mut stdout = stdout();
    print!("repl> ");
    stdout.flush();
    for line in stdin.lines() {
        read_eval(line.unwrap(), &mut env);

        print!("repl> ");
        stdout.flush();
    }

}

/// A convenience function that calls read & eval and displays their results
pub fn read_eval(s: String, env: &mut Env) {
    match read(s.as_ref()) {
        Err(e) => println!("\nParse error: {}", e),
        Ok(parsed) => {
            for expr in parsed.into_iter() {
                match eval(env, expr) {
                    Ok(Nil) => {},
                    Ok(expr) => expr.print(),
                    Err(ref x) => println!("\nError: {}", *x),
                }
            }
        }
    }
}

/// The representation of Lisp expressions
pub type Expr = Expression<String>;

#[derive(Clone)]
pub struct Env {
    bindings: HashMap<String, Expr>,
}

impl Env {
    fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    fn find_copy(&self, key: &String) -> Option<Expr> {
        self.bindings.get(key).cloned()
    }
}
