use std::{f64::consts::{E, PI}, iter::{Fuse, Peekable}, str::Chars};
use eyre::{Report, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
  Add,
  Sub,
  Mul,
  Div,
  Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
  Abs,
  Sqrt,
  Log(f64),
  Sin,
  Cos,
  Tg,
  Ctg,
  Asin,
  Acos,
  Atan,
  Exp,
  Root(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Literal(f64),
  Operator(Op),
  Function(Func),
  LeftBracket,
  RightBracket,
  End,
}

struct CharStream<'a> {
  index: usize,
  iterator: Peekable<Fuse<Chars<'a>>>,
}

impl<'a> CharStream<'a> {
  fn new(input: &'a str) -> CharStream<'a> {
    Self {
      index: 0,
      iterator: input.chars().fuse().peekable(),
    }
  }

  fn next(&mut self) -> char {
    self.index += 1;
    self.iterator.next().unwrap_or('\0')
  }

  fn peek(&mut self) -> char {
    self.iterator.peek().cloned().unwrap_or('\0')
  }
}

#[derive(Debug)]
pub struct Lexer {
  index: usize,
  tokens: Vec<Token>,
}

impl Lexer {
  fn parse_func_argument(stream: &mut CharStream) -> Result<f64> {
    match Self::parse_token(stream) {
      Ok(Token::Literal(base)) => Ok(base),
      _ => Err(Report::msg("Unable to parse function argument")),
    }
  }

  fn parse_token(stream: &mut CharStream) -> Result<Token> {
    const IDENTS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const DIGITS: &'static str = "1234567890.";
  
    let mut c = stream.peek();
  
    if IDENTS.contains(c) {
      let mut buffer = String::new();
  
      while IDENTS.contains(c) {
        buffer.push(c);
        stream.next();
        c = stream.peek();
      }
  
      return match buffer.to_ascii_lowercase().as_str() {
        "abs" => Ok(Token::Function(Func::Abs)),
        "sqrt" => Ok(Token::Function(Func::Sqrt)),
        "log" => Ok(Token::Function(Func::Log(Self::parse_func_argument(stream)?))),
        "sin" => Ok(Token::Function(Func::Sin)),
        "cos" => Ok(Token::Function(Func::Cos)),
        "tg" | "tan" => Ok(Token::Function(Func::Tg)),
        "ctg" | "cotan" => Ok(Token::Function(Func::Ctg)),
        "asin" | "arcsin" => Ok(Token::Function(Func::Asin)),
        "acos" | "arccos" => Ok(Token::Function(Func::Acos)),
        "atan" | "arctan" => Ok(Token::Function(Func::Atan)),
        "exp" => Ok(Token::Function(Func::Exp)),
        "root" => Ok(Token::Function(Func::Root(Self::parse_func_argument(stream)?))),

        "pi" => Ok(Token::Literal(PI)),
        "e" => Ok(Token::Literal(E)),
        "phi" => Ok(Token::Literal((1. + 5_f64.sqrt()) / 2.)),
        
        _ => Err(Report::msg("Unknown identifier")),
      }
    }
  
    if DIGITS.contains(c) {
      let mut buffer = String::new();
  
      while DIGITS.contains(c) {
        buffer.push(c);
        stream.next();
        c = stream.peek();
      }

      if buffer.len() == 0 && buffer.as_str().chars().next().unwrap() == '.' {
        return Err(Report::msg("Invalid numeric literal"));
      }
  
      return Ok(Token::Literal(buffer.parse()?));
    }
  
    stream.next();
  
    match c {
      '(' => Ok(Token::LeftBracket),
      ')' => Ok(Token::RightBracket),
      '+' => Ok(Token::Operator(Op::Add)),
      '-' => Ok(Token::Operator(Op::Sub)),
      '*' => Ok(Token::Operator(Op::Mul)),
      '/' => Ok(Token::Operator(Op::Div)),
      '^' => Ok(Token::Operator(Op::Pow)),
      _ => Err(Report::msg(format!("Unknown token ({})", c)))
    }
  }

  fn new(input: &str) -> Result<Self> {
    let mut stream = CharStream::new(input);
    let mut tokens = vec![];

    loop {
      let c = stream.peek();

      if c == '\0' {
        break
      }
      
      if c.is_whitespace() {
        stream.next();
      } else {
        let token = Self::parse_token(&mut stream)?;
        tokens.push(token);
      }
    }

    Ok(Self {
      index: 0,
      tokens
    })
  }

  pub fn peek(&self) -> Token {
    self.tokens.get(self.index).cloned().unwrap_or(Token::End)
  }

  pub fn next(&mut self) -> Token {
    let token = self.peek();
    self.index += 1;
    token
  }
}

pub fn tokenize(input: &str) -> Result<Lexer> {
  Lexer::new(input)
}

#[allow(dead_code, unused_imports)]
mod tests {
  use super::{tokenize, Func, Op, Token};

  fn test(input: &str, tokens: impl IntoIterator<Item = Token>) {
    let mut lexer = tokenize(input).unwrap();

    for token in tokens {
      assert_eq!(lexer.next(), token);
    }

    assert_eq!(lexer.next(), Token::End)
  }

  #[test]
  fn test_numbers() {
    let input = "1 .2 3. 4.5";
    let tokens = vec![
      Token::Literal(1.),
      Token::Literal(0.2),
      Token::Literal(3.),
      Token::Literal(4.5),
    ];

    test(input, tokens)
  }

  #[test]
  fn test_operators() {
    let input = "+ - * / ^";
    let tokens = vec![
      Token::Operator(Op::Add),
      Token::Operator(Op::Sub),
      Token::Operator(Op::Mul),
      Token::Operator(Op::Div),
      Token::Operator(Op::Pow),
    ];

    test(input, tokens)
  }

  #[test]
  fn text_functions() {
    let input = "sqrt log2 abs sin cos";
    let tokens = vec![
      Token::Function(Func::Sqrt),
      Token::Function(Func::Log(2.)),
      Token::Function(Func::Abs),
      Token::Function(Func::Sin),
      Token::Function(Func::Cos),
    ];

    test(input, tokens)
  }
}
