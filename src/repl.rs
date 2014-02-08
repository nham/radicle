use std::io::BufferedReader;
use std::io::stdin;
use std::io::stdio;

use eval::eval;
use super::{Environment, read_eval};

pub fn do_repl() {
    let env = Environment::new();
    let mut stdin = BufferedReader::new(stdin());
    print!("repl> ");
    stdio::flush();
    for line in stdin.lines() {
        read_eval(line, &env);
        print!("repl> ");
        stdio::flush();
    }

}
