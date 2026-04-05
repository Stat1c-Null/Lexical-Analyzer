use std::collections::VecDeque;

use crate::token::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  ch: char,
  line: i32,
  column: i32,
  at_line_start: bool,
  indent_stack: Vec<usize>,
  pending_tokens: VecDeque<Token>,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      input: input.to_string(),
      position: 0,
      read_position: 0,
      ch: '\0',
      line: 1,
      column: 0,
      at_line_start: true,
      indent_stack: vec![0],
      pending_tokens: VecDeque::new(),
    };
    lexer.read_char();
    lexer
  }

  pub fn next_token(&mut self) -> Token {
    if let Some(tok) = self.pending_tokens.pop_front() {
      return tok;
    }

    if self.at_line_start {
      self.handle_indentation();
      if let Some(tok) = self.pending_tokens.pop_front() {
        return tok;
      }
    }

    self.skip_whitespace();

    if self.ch == '\0' {
      if self.indent_stack.len() > 1 {
        self.indent_stack.pop();
        return Token {
          r#type: TokenType::DEDENT,
          literal: String::new(),
          line: self.line,
          column: 1,
        };
      }

      return Token {
        r#type: TokenType::EOF,
        literal: String::new(),
        line: self.line,
        column: self.column,
      };
    }

    if self.ch == '\n' {
      let tok = Token {
        r#type: TokenType::NEWLINE,
        literal: "\\n".to_string(),
        line: self.line,
        column: self.column,
      };
      self.line += 1;
      self.column = 0;
      self.read_char();
      self.at_line_start = true;
      return tok;
    }

    if Lexer::is_letter(self.ch) {
      let line = self.line;
      let column = self.column;
      let literal = self.read_identifier();
      return Token {
        r#type: lookup_ident(&literal),
        literal,
        line,
        column,
      };
    }

    if self.ch.is_ascii_digit() {
      let line = self.line;
      let column = self.column;
      let literal = self.read_number();
      let token_type = if literal.contains('.') {
        TokenType::FLOAT
      } else {
        TokenType::INT
      };
      return Token {
        r#type: token_type,
        literal,
        line,
        column,
      };
    }

    if self.ch == '"' || self.ch == '\'' {
      let quote_type = self.ch;
      let line = self.line;
      let column = self.column;
      let literal = self.read_string(quote_type);
      if self.ch == quote_type {
        self.read_char();
      }
      return Token {
        r#type: TokenType::STRING,
        literal,
        line,
        column,
      };
    }

    let tok = match self.ch {
      '=' => {
        if self.peek_char() == '=' {
          let column = self.column;
          self.read_char();
          Token {
            r#type: TokenType::EQ,
            literal: "==".to_string(),
            line: self.line,
            column,
          }
        } else {
          Token {
            r#type: TokenType::ASSIGN,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
          }
        }
      }
      '+' => Token {
        r#type: TokenType::PLUS,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '-' => Token {
        r#type: TokenType::MINUS,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '!' => {
        if self.peek_char() == '=' {
          let column = self.column;
          self.read_char();
          Token {
            r#type: TokenType::NOTEQ,
            literal: "!=".to_string(),
            line: self.line,
            column,
          }
        } else {
          Token {
            r#type: TokenType::BANG,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
          }
        }
      }
      '*' => Token {
        r#type: TokenType::ASTERISK,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '/' => Token {
        r#type: TokenType::SLASH,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '%' => Token {
        r#type: TokenType::PERCENT,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '<' => {
        if self.peek_char() == '=' {
          let column = self.column;
          self.read_char();
          Token {
            r#type: TokenType::LTE,
            literal: "<=".to_string(),
            line: self.line,
            column,
          }
        } else {
          Token {
            r#type: TokenType::LT,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
          }
        }
      }
      '>' => {
        if self.peek_char() == '=' {
          let column = self.column;
          self.read_char();
          Token {
            r#type: TokenType::GTE,
            literal: ">=".to_string(),
            line: self.line,
            column,
          }
        } else {
          Token {
            r#type: TokenType::GT,
            literal: self.ch.to_string(),
            line: self.line,
            column: self.column,
          }
        }
      }
      ',' => Token {
        r#type: TokenType::COMMA,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      ':' => Token {
        r#type: TokenType::COLON,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '.' => Token {
        r#type: TokenType::DOT,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '(' => Token {
        r#type: TokenType::LPAREN,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      ')' => Token {
        r#type: TokenType::RPAREN,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      '[' => Token {
        r#type: TokenType::LBRACKET,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      ']' => Token {
        r#type: TokenType::RBRACKET,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
      _ => Token {
        r#type: TokenType::ILLEGAL,
        literal: self.ch.to_string(),
        line: self.line,
        column: self.column,
      },
    };

    self.read_char();
    tok
  }

  pub fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();

    loop {
      let tok = self.next_token();
      let is_eof = tok.r#type == TokenType::EOF;
      tokens.push(tok);
      if is_eof {
        break;
      }
    }

    tokens
  }

  fn handle_indentation(&mut self) {
    let mut indent_width = 0usize;
    let line = self.line;

    while self.ch == ' ' || self.ch == '\t' {
      indent_width += if self.ch == '\t' { 4 } else { 1 };
      self.read_char();
    }

    // Keep blank lines as NEWLINE tokens and preserve start-of-line state.
    if self.ch == '\n' || self.ch == '\0' {
      return;
    }

    let current_indent = *self.indent_stack.last().unwrap_or(&0);
    if indent_width > current_indent {
      self.indent_stack.push(indent_width);
      self.pending_tokens.push_back(Token {
        r#type: TokenType::INDENT,
        literal: String::new(),
        line,
        column: 1,
      });
    } else if indent_width < current_indent {
      while let Some(&top) = self.indent_stack.last() {
        if top <= indent_width {
          break;
        }

        self.indent_stack.pop();
        self.pending_tokens.push_back(Token {
          r#type: TokenType::DEDENT,
          literal: String::new(),
          line,
          column: 1,
        });
      }

      if *self.indent_stack.last().unwrap_or(&0) != indent_width {
        self.pending_tokens.push_back(Token {
          r#type: TokenType::ILLEGAL,
          literal: "inconsistent indentation".to_string(),
          line,
          column: 1,
        });
      }
    }

    self.at_line_start = false;
  }

  pub fn skip_whitespace(&mut self) {
    while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' {
      self.read_char();
    }
  }

  pub fn read_number(&mut self) -> String {
    let start_position = self.position;
    let mut has_dot = false;

    while self.ch.is_ascii_digit() || (!has_dot && self.ch == '.') {
      if self.ch == '.' {
        has_dot = true;
      }
      self.read_char();
    }

    self.input[start_position..self.position].to_string()
  }

  pub fn read_string(&mut self, quote_type: char) -> String {
    let start_position = self.read_position;

    loop {
      self.read_char();
      if self.ch == quote_type || self.ch == '\0' || self.ch == '\n' {
        break;
      }
    }

    self.input[start_position..self.position].to_string()
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

  pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
  }

  pub fn read_identifier(&mut self) -> String {
    let start_position = self.position;
    while Lexer::is_letter(self.ch) || self.ch.is_ascii_digit() {
      self.read_char();
    }
    self.input[start_position..self.position].to_string()
  }
}