use crate::lexer::{
    Lexer,
    token::{Token, TokenKind},
};
use anyhow::Result;
pub fn handle_comments(line: u16, lexer: &mut Lexer) -> Result<Token> {
    lexer.expect('/')?;
    lexer.expect('/')?;

    let mut value = String::new();
    while lexer.current() != '\n' {
        value += &lexer.advance().to_string();
    }

    Ok(Token {
        value,
        kind: TokenKind::Comment,
        line,
    })
}

pub fn handle_number(line: u16, lexer: &mut Lexer) -> Result<Token> {
    let mut value = String::new();
    while lexer.valid_number_token_chars.contains(&lexer.current()) {
        value += &lexer.advance().to_string();
    }

    Ok(Token {
        value,
        kind: TokenKind::Number,
        line,
    })
}

pub fn handle_identifier(line: u16, lexer: &mut Lexer) -> Result<Token> {
    let mut value = String::new();
    while lexer
        .valid_identifier_token_chars
        .contains(&lexer.current())
    {
        value += &lexer.advance().to_string();
    }

    match lexer.keywords.get(value.as_str()) {
        Some(token_kind) => Ok(Token {
            value: String::new(),
            kind: *token_kind,
            line,
        }),
        None => Ok(Token {
            value,
            kind: TokenKind::Identifier,
            line,
        }),
    }
}

pub fn handle_compiler_data(line: u16, lexer: &mut Lexer) -> Result<Token> {
    lexer.expect('#')?;

    let mut value = String::new();
    while lexer.current() != '\n' {
        value += &lexer.advance().to_string();
    }

    Ok(Token {
        value,
        kind: TokenKind::CompilerData,
        line,
    })
}

pub fn handle_string(line: u16, lexer: &mut Lexer) -> Result<Token> {
    lexer.expect('"')?;

    let mut value = String::new();
    while lexer.current() != '"' {
        value += &lexer.advance().to_string();
    }
    lexer.expect('"')?;

    Ok(Token {
        value,
        kind: TokenKind::String,
        line,
    })
}
