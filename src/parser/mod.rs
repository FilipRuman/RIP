pub mod expression;
pub mod parsing_functions;
mod token_stats;
pub mod types;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail};
use log::*;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        expression::Expression,
        parsing_functions::identifier_parsing,
        token_stats::{TokenStats, token_stats},
    },
};

type NodFunction = fn(&mut Parser) -> Result<Expression>;
type LedFunction = fn(&mut Parser, left: Expression, bp: i8) -> Result<Expression>;

pub struct Parser {
    pub valid_data_names: HashSet<&'static str>,
    pub tokens: Vec<Token>,
    pub i: usize,
    pub token_stats: HashMap<TokenKind, TokenStats>,
    pub file: String,
}
impl Parser {
    pub fn debug_data(&self) -> expression::DebugData {
        expression::DebugData {
            file: self.file.to_owned(),
            line: self.current().line,
        }
    }
    pub fn advance(&mut self) -> &Token {
        self.i += 1;
        &self.tokens[self.i - 1]
    }

    pub fn next(&self) -> &Token {
        &self.tokens[self.i + 1]
    }
    pub fn current(&self) -> &Token {
        &self.tokens[self.i]
    }

    pub fn current_stats(&self) -> Result<&TokenStats> {
        self.token_stats.get(&self.current().kind).with_context(|| {
            format!(
                "there were no stats data for token of kind: '{:?}'",
                self.current().kind
            )
        })
    }

    #[must_use]
    pub fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        match self.advance().to_owned() {
            val => {
                if val.kind == expected {
                    return Ok(val);
                } else {
                    bail!("expected to find token of kind: '{expected:?}', found: '{val:?}'");
                }
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>, file: String) -> Result<Vec<Expression>> {
    let mut parser = Parser {
        valid_data_names: HashSet::from([
            "bool", "char", "short", "int", "long", "float", "double",
        ]),
        i: 0,
        tokens: tokens,
        token_stats: token_stats(),
        file,
    };

    let mut output = vec![];
    while parser.current().kind != TokenKind::EndOfFile {
        if parser.current().kind == TokenKind::SemiColon {
            parser.advance();
            continue;
        }
        output.push(
            parsing_functions::expression(&mut parser, 0).with_context(|| {
                format!(
                    "debug data: {:?},\n parsed expressions:{:#?}",
                    parser.debug_data(),
                    output
                )
            })?,
        );
    }
    Ok(output)
}
