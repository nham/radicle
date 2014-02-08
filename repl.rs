use std::io::BufferedReader;
use std::io::stdin;
use std::io::stdio;
use std::hashmap::HashMap;

use super::{Environment, eval, read, read_eval};
use Atom = tree::Leaf;
use List = tree::Branch;

pub fn do_repl() {
    let env = Environment { parent: None, bindings: HashMap::new() };
    let mut stdin = BufferedReader::new(stdin());
    print!("repl> ");
    stdio::flush();
    for line in stdin.lines() {
        read_eval(line, &env);
        print!("repl> ");
        stdio::flush();
    }

}
