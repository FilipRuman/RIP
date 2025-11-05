use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail};
use log::warn;

use crate::lexer::{
    patterns::{TokenPattern, setup_token_patters},
    token::{Token, TokenKind},
};

mod patterns;
pub mod token;
mod tokenization_functions;

pub struct Lexer {
    contents: Vec<char>,
    pub i: usize,
    token_patterns: HashMap<(char, char), TokenPattern>,
    keywords: HashMap<&'static str, TokenKind>,
    pub valid_identifier_token_chars: HashSet<char>,
    pub valid_number_token_chars: HashSet<char>,
}
impl Lexer {
    pub fn next(&self) -> char {
        match self.contents.get(self.i + 1) {
            Some(val) => *val,
            None => '\n',
        }
    }
    pub fn current(&self) -> char {
        match self.contents.get(self.i) {
            Some(val) => *val,
            None => '\n',
        }
    }
    #[must_use]
    pub fn expect(&mut self, expected: char) -> Result<char> {
        let current = self.advance();
        if current != expected {
            bail!("Expected: '{expected}', found: '{current}'")
        }
        Ok(current)
    }
    pub fn advance(&mut self) -> char {
        self.i += 1;
        match self.contents.get(self.i - 1) {
            Some(val) => *val,
            None => '\n',
        }
    }
}

pub fn tokenize(text: Vec<char>) -> Result<Vec<Token>> {
    let valid_identifier_token_chars: HashSet<char> = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
    ]
    .into_iter()
    .collect();
    let valid_number_token_chars: HashSet<char> = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'x', 'a', 'b', 'c', 'd', 'e', 'f',
    ]
    .into_iter()
    .collect();
    let keywords: HashMap<&str, TokenKind> = HashMap::from([
        ("if", TokenKind::If),
        ("else", TokenKind::Else),
        ("break", TokenKind::Break),
        ("return", TokenKind::Return),
        ("while", TokenKind::While),
        ("static", TokenKind::Static),
        ("const", TokenKind::Constant),
        ("enum", TokenKind::Enum),
        ("true", TokenKind::True),
        ("false", TokenKind::False),
        ("struct", TokenKind::Struct),
        ("for", TokenKind::For),
    ]);

    let mut lexer = Lexer {
        keywords,
        valid_identifier_token_chars,
        valid_number_token_chars,
        contents: text,
        i: 0,
        token_patterns: setup_token_patters().context("setting up token patterns")?,
    };

    let mut output: Vec<Token> = vec![];
    let mut current_line: u16 = 0;
    while lexer.i < lexer.contents.len() {
        let pattern = match patterns::pattern_for_current_char(&mut lexer) {
            Some(val) => val,
            None => {
                warn!(
                    "there was no pattern for combo: '{}'&&'{}'",
                    lexer.current(),
                    lexer.next()
                );
                lexer.advance();
                continue;
            }
        };

        output.push(match pattern {
            TokenPattern::Fast {
                kind,
                use_second_char,
            } => {
                if kind == TokenKind::NextLine {
                    current_line += 1;
                }
                // advance
                lexer.i += 1 + use_second_char as usize;
                Token {
                    value: String::new(),
                    kind,
                    line: current_line,
                }
            }
            TokenPattern::Long(function) => function(current_line, &mut lexer)
                .context("while running a token pattern function")?,
        });
    }

    Ok(output)
}
