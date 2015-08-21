extern crate rustc_serialize;
extern crate docopt;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use docopt::Docopt;
mod interpreter;

const USAGE: &'static str = "
Crappy Lang.

Usage:
  crappy <filename>
  crappy
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_filename: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.decode())
        .unwrap_or_else(|e| e.exit());

    if args.arg_filename != "" {
      let mut file = File::open(args.arg_filename).unwrap();

      let mut contents = String::new();

      file.read_to_string(&mut contents).unwrap();
      
      let lines: Vec<&str> = contents.split("\n").collect();

      let mut interpreter = interpreter::Interpreter::new();

      for line in lines {
          interpreter.interpret(&line.to_string());
      }
    } else {
      println!("Crappy Lang REPL v0.0.1");
  
      let mut interpreter = interpreter::Interpreter::new();
      let mut input_line = String::new();

      
      loop {
        print!("> ");
        io::stdout().flush();

        io::stdin().read_line(&mut input_line).ok().expect("The read line failed");
        
        match input_line.trim() {
          "exit" => break,
          line => interpreter.interpret(&line.to_string())
        }
        
        input_line.clear();
      }
    }
}
