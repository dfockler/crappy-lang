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
      "join" => {
        let first = tokens[1];
        let second = tokens[2];
        let mut first_values = self.memory_map.get(first).unwrap().clone();
        let second_values = self.memory_map.get(second).unwrap().clone();

        for value in second_values {
          first_values.push(value);
        }

        if tokens.len() == 5 {
          self.memory_map.insert(tokens[4], first_values);
        }
      },
      "same" => {
        let first = tokens[1];
        let second = tokens[2];
        let first_values = self.memory_map.get(first).unwrap().clone();
        let second_values = self.memory_map.get(second).unwrap().clone();
        let mut new_values: Vec<i32> = Vec::new();

        for value in &second_values {
          if first_values.contains(&value) {
            new_values.push(*value);
          }
        }

        for value in &first_values {
          if second_values.contains(&value) && !new_values.contains(&value) {
            new_values.push(*value);
          }
        }

        if tokens.len() == 5 {
          self.memory_map.insert(tokens[4], new_values);
        } 
      },
      "diff" => {
        let first = tokens[1];
        let second = tokens[2];
        let first_values = self.memory_map.get(first).unwrap().clone();
        let second_values = self.memory_map.get(second).unwrap().clone();
        let mut new_values: Vec<i32> = Vec::new();

        for value in &second_values {
          if !first_values.contains(&value) {
            new_values.push(*value);
          }
        }

        for value in &first_values {
          if !second_values.contains(&value) && !new_values.contains(&value) {
            new_values.push(*value);
          }
        }

        if tokens.len() == 5 {
          self.memory_map.insert(tokens[4], new_values);
        } 
      },
      "scale" => {
        let name = tokens[1];
        let scalar = tokens[2].parse::<i32>().unwrap();
        let values = self.memory_map.get(name).unwrap().clone();
        let mut new_values: Vec<i32> = Vec::new();

        for value in &values {
          new_values.push(*value * scalar);
        }

        if tokens.len() == 5 {
          self.memory_map.insert(tokens[4], new_values);
        }
      },
      "out" => {
        let name = tokens[1];
        let values = self.memory_map.get(name).unwrap().clone();

        print!("{}: ", name);
        for (index, value) in values.iter().enumerate() {
          if index == values.len() - 1 {
            print!("{}", value);
          }
          else {
            print!("{}, ", value);
          }
        }
        println!("");
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