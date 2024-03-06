use crate::lexer::{Func, Lexer, Op, Token};
use eyre::{Report, Result};

#[derive(Debug)]
pub enum Node {
  Immediate(f64),
  BinOp(Op, Box<Node>, Box<Node>),
  Func(Func, Box<Node>),
}

/// `bracket: bool` is used to only parse bracketed expressions when dealing with functions
/// Otherwise "abs-2" may count as a valid expression
fn parse_primary(lexer: &mut Lexer, bracket: bool) -> Result<Node> {
  match lexer.next() {
    Token::Operator(Op::Sub) if !bracket => {
      let value = parse_primary(lexer, false)?;
      Ok(Node::Immediate(-value.evaluate()?))
    },
    Token::Literal(value) if !bracket => Ok(Node::Immediate(value)),
    Token::LeftBracket => {
      let value = parse_expression(lexer)?;
      match lexer.next() {
        Token::RightBracket => Ok(value),
        _ => Err(Report::msg("Parenthesis don't match")),
      }
    },
    _ => Err(Report::msg("Unexpected token"))
  }
}

fn parse_func(lexer: &mut Lexer) -> Result<Node> {
  if let Token::Function(func) = lexer.peek() {
    lexer.next();
    let arg = parse_primary(lexer, true)?;
    return Ok(Node::Func(func, Box::new(arg)))
  }

  parse_primary(lexer, false)
}

fn parse_power(lexer: &mut Lexer) -> Result<Node> {
  let mut left = parse_func(lexer)?;

  loop {
    match lexer.peek() {
      Token::Operator(Op::Pow) => {
        lexer.next();
        let right = parse_func(lexer)?;
        left = Node::BinOp(Op::Pow, Box::new(left), Box::new(right));
      }
      _ => break Ok(left),
    }
  }
}

fn parse_multiplicative(lexer: &mut Lexer) -> Result<Node> {
  let mut left = parse_power(lexer)?;

  loop {
    match lexer.peek() {
      Token::Operator(op) if op == Op::Mul || op == Op::Div => {
        lexer.next();
        let right = parse_power(lexer)?;
        left = Node::BinOp(op, Box::new(left), Box::new(right));
      }
      _ => break Ok(left),
    }
  }
}

fn parse_additive(lexer: &mut Lexer) -> Result<Node> {
  let mut left = parse_multiplicative(lexer)?;

  loop {
    match lexer.peek() {
      Token::Operator(op) if op == Op::Add || op == Op::Sub => {
        lexer.next();
        let right = parse_multiplicative(lexer)?;
        left = Node::BinOp(op, Box::new(left), Box::new(right))
      }
      Token::End => break Ok(left),
      _ => break Err(Report::msg("Unexpected token"))
    }
  } 
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Node> {
  parse_additive(lexer)
}
