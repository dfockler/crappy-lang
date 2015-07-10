pub fn interpret(line: &str) {
  tokenize(line);
  // Execute
  // Output Execution Results
}

fn tokenize(line: &str) {
  let tokens = line.split(" ").collect::<Vec<&str>>();

  if tokens[0] == "set" {
    set(tokens[1], parse_set_values(line));
  }
}

fn set(variable: &str, values: Vec<i32>) {
  print!("{0} -> ", variable);
  for value in values {
    print!("{:?}, ", value)
  }
  println!("");
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