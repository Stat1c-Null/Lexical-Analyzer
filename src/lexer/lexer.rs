use crate::token::token::{Token, TokenType};

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  ch: char,
  line: i32,
  column: i32,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
        read_position: 0,
        ch: '\0',
        line: 1,
        column: 1,
    };
      //lexer.read_char();
      lexer
    }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();

    let mut tok = Token {
        r#type: TokenType::ILLEGAL,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
    };

    match self.ch {
      '=' => {
        if self.peek_char() == '=' {
            let ch = self.ch;
            self.read_char();
            tok = Token {
                r#type: TokenType::EQ,
                literal: format!("{}{}", ch, self.ch),
                line: self.line,
                column: self.column - 1,
            };
        } else {
            tok = Token {
                r#type: TokenType::ASSIGN,
                literal: self.ch.to_string(),
                line: self.line,
                column: self.column,
            };
        }
      }
      '+' => {
          tok = Token {
              r#type: TokenType::PLUS,
              literal: self.ch.to_string(),
              line: self.line,
              column: self.column,
          };
      }
      '(' => {
        tok = Token {
            r#type: TokenType::LPAREN,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      ')' => {
        tok = Token {
            r#type: TokenType::RPAREN,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      '-' => {
          tok = Token {
              r#type: TokenType::MINUS,
              literal: self.ch.to_string(),
              line: self.line,
              column: self.column,
          };
      }
      '*' => {
          tok = Token {
              r#type: TokenType::ASTERISK,
              literal: self.ch.to_string(),
              line: self.line,
              column: self.column,
          };
      }
      '/' => {
          tok = Token {
              r#type: TokenType::SLASH,
              literal: self.ch.to_string(),
              line: self.line,
              column: self.column,
          };
      }
      '!' => {
        if self.peek_char() == '=' {
            let ch = self.ch;
            self.read_char();
            tok = Token {
                r#type: TokenType::NOTEQ,
                literal: format!("{}{}", ch, self.ch),
                line: self.line,
                column: self.column - 1,
            };
        } else {
            tok = Token {
                r#type: TokenType::ILLEGAL,
                literal: self.ch.to_string(),
                line: self.line,
                column: self.column,
            };
        }
      }
      '<' => {
        if self.peek_char() == '=' {
            let ch = self.ch;
            self.read_char();
            tok = Token {
                r#type: TokenType::LTE,
                literal: format!("{}{}", ch, self.ch),
                line: self.line,
                column: self.column - 1,
            };
        } else {
            tok = Token {
                r#type: TokenType::LT,
                literal: self.ch.to_string(),
                line: self.line,
                column: self.column,
            };
        }
      }
      '>' => {
        if self.peek_char() == '=' {
            let ch = self.ch;
            self.read_char();
            tok = Token {
                r#type: TokenType::GTE,
                literal: format!("{}{}", ch, self.ch),
                line: self.line,
                column: self.column - 1,
            };
        } else {
            tok = Token {
                r#type: TokenType::GT,
                literal: self.ch.to_string(),
                line: self.line,
                column: self.column,
            };
        }
      }
      ',' => {
        tok = Token {
            r#type: TokenType::COMMA,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      '[' => {
        tok = Token {
            r#type: TokenType::LBRACKET,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      ']' => {
        tok = Token {
            r#type: TokenType::RBRACKET,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      ':' => {
        tok = Token {
            r#type: TokenType::COLON,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
        };
      }
      '"' => {
        let quote_type = self.ch;
        let start_column = self.column;
        let mut literal = String::new();
        loop {
            self.read_char();
            if self.ch == quote_type || self.ch == '\0' {
                break;
            }
            literal.push(self.ch);
        }
        tok = Token {
            r#type: TokenType::STRING,
            literal,
            line: self.line,
            column: start_column,
        };
      }
      '0'..='9' => {
        let start_column = self.column;
        let mut literal = String::new();
        while self.ch.is_digit(10) {
            literal.push(self.ch);
            self.read_char();
        }
        tok = Token {
            r#type: TokenType::INT,
            literal,
            line: self.line,
            column: start_column,
        };
        return tok; // Return early to avoid read_char() at the end
      }


      _ => {}
    }
    tok
  }

  pub fn skip_whitespace(&mut self) {
    while self.ch.is_whitespace() {
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        }
        self.read_char();
    }
  }

  pub fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
        self.ch = '\0';
    } else {
        self.ch = self.input.as_bytes()[self.read_position] as char;
    }
    self.position = self.read_position;
    self.read_position += 1;
    self.column += 1;
  }

  pub fn peek_char(&self) -> char {
    if self.read_position >= self.input.len() {
        '\0'
    } else {
        self.input.as_bytes()[self.read_position] as char
    }
  }
}