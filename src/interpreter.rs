enum TokenType {
  method,
  variable,
  set,
  primitive
}

struct Token {
  ttype: TokenType,
  value: String
}

pub fn interpret_line(line: &str) {
  tokenize(line);
}

fn interpret(line: &str) {
  // Tokenize
  // Execute
  // Output Execution Results
}

fn tokenize(line: &str) {
  let tokens = line.split(' ');
  for token in tokens {
    println!("{:?}", token);
  }
}