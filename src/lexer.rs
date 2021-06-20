use std::{u64, usize};

use crate::token::{self, TokenType};

struct Lexer {
    input: String,
    position: usize,      // Current position in input (points to current char)
    read_position: usize, // Current reading position in input (after current char)
    ch: Option<char>,     // Current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> TokenType {
        self.read_char();

        match self.ch {
            Some('=') => TokenType::Assign,
            Some(';') => TokenType::Semicolon,
            Some('{') => TokenType::Lbrace,
            Some('}') => TokenType::Rbrace,
            Some(',') => TokenType::Comma,
            Some('+') => TokenType::Plus,
            Some('(') => TokenType::Lparen,
            Some(')') => TokenType::Rparen,
            None => TokenType::Eof,
            _ => TokenType::Illegal,
        }
    }
}

#[test]
fn next_token_test() {
    let input = "=+(){},;";

    let tests = vec![
        TokenType::Assign,
        TokenType::Plus,
        TokenType::Lparen,
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Rbrace,
        TokenType::Comma,
        TokenType::Semicolon,
    ];

    let mut l = Lexer::new(input.to_string());
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}
