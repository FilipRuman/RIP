mod lexer;
pub mod parser;

use std::{
    collections::HashSet,
    fs::{self},
};

//INFO: Default includes: use log::*;use anyhow::{Result,bail,Context};
use anyhow::{Context, Result};
use log::*;

use crate::lexer::token::{Token, TokenKind};

fn main() {
    colog::init();
    info!("init colog");
    if let Err(err) = parse() {
        error!("{err:?}")
    }
}

fn parse() -> Result<()> {
    const FILE_PATH: &str = "test_files/test.c";

    let mut tokens = tokenize_file(FILE_PATH)
        .with_context(|| format!("tokenization of a file at path: '{FILE_PATH}'"))?;

    black_list_filter_tokens_by_kind(
        &mut tokens,
        HashSet::from([
            TokenKind::Tab,
            TokenKind::Comment,
            TokenKind::NextLine,
            TokenKind::WhiteSpace,
        ]),
    );

    tokens.push(Token {
        value: String::new(),
        kind: TokenKind::EndOfFile,
        line: 0,
    });
    info!("Tokens: {tokens:#?}");

    let expressions = parser::parse(tokens, FILE_PATH.to_owned())?;

    info!("Expressions: {expressions:#?}");

    Ok(())
}

fn tokenize_file(path: &str) -> Result<Vec<Token>> {
    let chars = fs::read_to_string(path)?.chars().collect::<Vec<char>>();
    lexer::tokenize(chars)
}

fn black_list_filter_tokens_by_kind(tokens: &mut Vec<Token>, black_list: HashSet<TokenKind>) {
    tokens.retain(|token| !black_list.contains(&token.kind))
}
