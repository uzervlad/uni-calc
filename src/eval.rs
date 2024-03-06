use crate::{lexer::{Func, Op}, parser::Node};
use eyre::{eyre, Result};


impl Op {
  pub fn evaluate(self, left: f64, right: f64) -> Result<f64> {
    match self {
      Op::Add => Ok(left + right),
      Op::Sub => Ok(left - right),
      Op::Mul => Ok(left * right),
      Op::Div => {
        if right == 0.0 {
          Err(eyre!("Invalid operation: division by zero"))
        } else {
          Ok(left / right)
        }
      },
      Op::Pow => Ok(left.powf(right)),
    }
  }
}

impl Func {
  pub fn evaluate(self, arg: f64) -> Result<f64> {
    match self {
      Func::Abs => Ok(arg.abs()),
      Func::Sqrt => {
        if arg < 0.0 {
          Err(eyre!("Invalid operation: square root of negative number"))
        } else {
          Ok(arg.sqrt())
        }
      }
      Func::Log(base) => {
        // Use log2 or log10 if possible for better accuracy
        if base == 2. {
          Ok(arg.log2())
        } else if base == 10. {
          Ok(arg.log10())
        } else {
          Ok(arg.log(base))
        }
      },
      Func::Cos => Ok(arg.cos()),
      Func::Sin => Ok(arg.sin()),
      Func::Tg => Ok(arg.tan()),
      Func::Ctg => Op::Div.evaluate(1.0, arg.tan()),
      Func::Asin => {
        if arg < -1.0 || arg > 1.0 {
          Err(eyre!("Invalid operation: arcsine out of range"))
        } else {
          Ok(arg.asin())
        }
      },
      Func::Acos => {
        if arg < -1.0 || arg > 1.0 {
          Err(eyre!("Invalid operation: arccosine out of range"))
        } else {
          Ok(arg.acos())
        }
      }
      Func::Atan => Ok(arg.atan()),
      Func::Exp => Ok(arg.exp()),
      Func::Root(base) => Ok(arg.powf(Op::Div.evaluate(1.0, base)?)),
    }
  }
}

impl Node {
  pub fn evaluate(self) -> Result<f64> {
    match self {
      Node::Immediate(value) => Ok(value),
      Node::BinOp(op, left, right) => op.evaluate(left.evaluate()?, right.evaluate()?),
      Node::Func(func, node) => func.evaluate(node.evaluate()?),
    }
  }
}

#[allow(dead_code)]
mod tests {
  use crate::{lexer::tokenize, parser::parse_expression};

  fn test(input: &str, expected: f64) {
    let mut lexer = tokenize(input).unwrap();
    let ast = parse_expression(&mut lexer).unwrap();
    let result = ast.evaluate().unwrap();

    assert_eq!(result, expected)
  }

  #[test]
  fn test_operators() {
    test("1 + 2 - 4", -1.);
    test("5 - 1 / 2", 4.5);
    test("2 * 3 ^ 2", 18.);
  }

  #[test]
  fn test_functions() {
    test("sqrt(abs(-2))", 2_f64.sqrt());
    test("cos(pi)", -1.);
    test("sin(log2(10))", 10_f64.log(2.0).sin());
  }
}
