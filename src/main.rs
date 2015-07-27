use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
mod interpreter;

fn main() {
    let filename = env::args().nth(1).expect("You need an input file");

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut interpreter = interpreter::Interpreter::new();

    for line in lines {
        interpreter.interpret(line);
    }

    interpreter.memory();
}
