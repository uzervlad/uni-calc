use crate::{lexer::{Func, Op}, parser::Node};

impl Op {
  pub fn evaluate(self, left: f64, right: f64) -> f64 {
    match self {
      Op::Add => left + right,
      Op::Sub => left - right,
      Op::Mul => left * right,
      Op::Div => left / right,
      Op::Pow => left.powf(right),
    }
  }
}

impl Func {
  pub fn evaluate(self, arg: f64) -> f64 {
    match self {
      Func::Abs => arg.abs(),
      Func::Sqrt => arg.sqrt(),
      Func::Log(base) => arg.log(base),
      Func::Cos => arg.cos(),
      Func::Sin => arg.sin(),
      Func::Tg => arg.tan(),
      Func::Ctg => 1.0 / arg.tan(),
      Func::Asin => arg.asin(),
      Func::Acos => arg.acos(),
      Func::Atan => arg.atan(),
      Func::Exp => arg.exp(),
      Func::Root(base) => arg.powf(1. / base),
    }
  }
}

impl Node {
  pub fn evaluate(self) -> f64 {
    match self {
      Node::Immediate(value) => value,
      Node::BinOp(op, left, right) => op.evaluate(left.evaluate(), right.evaluate()),
      Node::Func(func, node) => func.evaluate(node.evaluate()),
    }
  }
}

#[allow(dead_code)]
mod tests {
  use crate::{lexer::tokenize, parser::parse_expression};

  fn test(input: &str, expected: f64) {
    let mut lexer = tokenize(input).unwrap();
    let ast = parse_expression(&mut lexer).unwrap();
    let result = ast.evaluate();

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
