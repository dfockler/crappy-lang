#![feature(collections)]
use std::collections::HashMap;

pub struct Interpreter<'a> {
  memory_map: HashMap<&'a str, Vec<i32>>
}

impl<'a> Interpreter<'a> {

  pub fn new() -> Interpreter<'a> {
    Interpreter { memory_map: HashMap::new() }
  }

  pub fn interpret(&mut self, line: &'a str) {
    let tokens = line.split(" ").collect::<Vec<&str>>();

    match tokens[0] {
      "set" => {
        let set_values = parse_set_values(line);
        let name = tokens[1];
        self.memory_map.insert(name, set_values);
      },
      _ => {
        println!("Unknown Definition");
      }
    }
  }

  pub fn memory(&mut self) {
    for (name, values) in &self.memory_map {
      print!("{}:", name);
      for value in values {
        print!("{}, ", value);
      }
      println!("");
    }
  }
}

fn parse_set_values(line: &str) -> Vec<i32> {
  let start = '(';
  let end = ')';

  let nums: String = line.chars()
  .skip_while(|&c| c != start).skip(1)
  .take_while(|&c| c != end)
  .collect();

  nums.split(',').map(|num|
    num.trim().parse::<i32>().unwrap()
  ).collect()
}