use std::io::{stdin, stdout, Write as _};
use eyre::Result;

use crate::{lexer::tokenize, parser::parse_expression};

mod eval;
mod lexer;
mod parser;

trait RoundWithPrecision {
  fn round_with_precision(&self, precision: u32) -> Self;
}

impl RoundWithPrecision for f64 {
  fn round_with_precision(&self, precision: u32) -> Self {
    let m = 10_i32.pow(precision) as f64;
    (self * m).round() / m
  }
}

fn main() -> Result<()> {
  let mut input = String::new();

  println!("Calculator. Use \"funcs\", \"ops\", or \"consts\" for help.");
  println!("\"exit\" to exit");

  loop {
    input.clear();

    print!("> ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    if let Some('\n') = input.chars().next_back() {
      input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
      input.pop();
    }

    match input.as_str() {
      "" => {
        println!("Author: Гаврилович Владислав");
        println!("For help, type \"funcs\", \"ops\", or \"consts\"");
      },
      "funcs" => {
        println!("Available functions:");
        println!("* abs(x) - absolute value");
        println!("* sqrt(x) - square root");
        println!("* logX(y) - base X logarithm of y");
        println!("* sin(x) - sine");
        println!("* cos(x) - cosine");
        println!("* tg(x) / tan(x) - tangent");
        println!("* ctg(x) / cotan(x) - cotangent");
        println!("* asin(x) / arcsin(x) - arcsine");
        println!("* acos(x) / arccos(x) - arccosine");
        println!("* atan(x) / arctan(x) - arctangent");
        println!("* exp(x) - exponent (e^x)");
        println!("* rootX(y) - root of y with base X");
      },
      "ops" => {
        println!("Available operators:");
        println!("* Addition (+)");
        println!("* Subtraction (-)");
        println!("* Multiplication (*)");
        println!("* Division (/)");
        println!("* Power (^)");
      },
      "consts" => {
        println!("Available constants:");
        println!("* pi - 3.14159...");
        println!("* e - 2.71828...");
        println!("* phi - golden ratio (1.61803...)");
      },
      "exit" => break,
      input => {
        let mut lexer = match tokenize(input) {
          Ok(lexer) => lexer,
          Err(report) => {
            println!("Error during tokenization: {:?}", report);
            continue
          }
        };
        let ast = match parse_expression(&mut lexer) {
          Ok(ast) => ast,
          Err(report) => {
            println!("Error during AST construction: {:?}", report);
            continue
          }
        };
        let result = ast.evaluate();
      
        println!("{}", result.round_with_precision(5));
      }
    }
  }  

  Ok(())
}
