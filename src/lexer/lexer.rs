use crate::token::token::{Token, TokenType};

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  ch: Option<char>,
  line: i32,
  column: i32,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
        read_position: 0,
        ch: None,
        line: 1,
        column: 1,
    };
      //lexer.read_char();
      lexer
    }

  pub fn next_token(&mut self) -> Vec<Token> {
    return vec![];
  }
}