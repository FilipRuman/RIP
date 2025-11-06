pub mod expression;
pub mod identifier_parsing;
pub mod parsing_functions;
pub mod types;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail};
use log::*;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{DebugData, Expression},
        parsing_functions::binary_expression,
    },
};

type NodFunction = fn(&mut Parser) -> Result<Expression>;
type LedFunction = fn(&mut Parser, left: Expression, bp: i8) -> Result<Expression>;
pub struct TokenStats {
    pub binding_power: i8,
    pub nod_function: Option<NodFunction>,
    pub led_function: Option<LedFunction>,
}

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

fn token_stats() -> HashMap<TokenKind, TokenStats> {
    HashMap::from([
        // CompilerData,
        // OpenParen,
        // CloseParen,
        // OpenBracket,
        // CloseBracket,
        // OpenCurly,
        // CloseCurly,
        // Comma,
        // Dot,

        // Colon,
        // Arrow,
        // Question,
        (
            TokenKind::EndOfFile,
            TokenStats {
                binding_power: -1,
                nod_function: None,
                led_function: None,
            },
        ),
        (
            TokenKind::SemiColon,
            TokenStats {
                binding_power: -1,
                nod_function: None,
                led_function: None,
            },
        ),
        (
            TokenKind::BitwiseShiftLeft,
            TokenStats {
                binding_power: 2,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::BitwiseShiftRight,
            TokenStats {
                binding_power: 2,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Plus,
            TokenStats {
                binding_power: 2,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Minus,
            TokenStats {
                binding_power: 2,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Star,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Slash,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Percent,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Equals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::NotEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Less,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::LessEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Greater,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::GreaterEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Or,
            TokenStats {
                binding_power: 1,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::And,
            TokenStats {
                binding_power: 1,
                nod_function: None,
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Not,
            TokenStats {
                binding_power: 1,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary_expression),
            },
        ),
        (
            TokenKind::Number,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::number),
                led_function: None,
            },
        ),
        (
            TokenKind::String,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::string),
                led_function: None,
            },
        ),
        (
            TokenKind::Identifier,
            TokenStats {
                binding_power: 0,
                nod_function: Some(identifier_parsing::identifier),
                led_function: None,
            },
        ),
        (
            TokenKind::Return,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::return_expr),
                led_function: None,
            },
        ),
        (
            TokenKind::CloseParen,
            TokenStats {
                binding_power: 0,
                nod_function: None,
                led_function: None,
            },
        ),
        (
            TokenKind::OpenParen,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::function_call),
            },
        ),
        (
            TokenKind::True,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::boolean),
                led_function: None,
            },
        ),
        (
            TokenKind::False,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::boolean),
                led_function: None,
            },
        ),
        (
            TokenKind::CloseCurly,
            TokenStats {
                binding_power: 0,
                nod_function: None,
                led_function: None,
            },
        ),
        (
            TokenKind::CompilerData,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::compiler_data),
                led_function: None,
            },
        ),
        (
            TokenKind::OpenCurly,
            TokenStats {
                binding_power: 0,
                nod_function: None,
                led_function: None,
            },
        ),
        // PlusEquals,
        // MinusEquals,
        // StarEquals,
        // SlashEquals,
        // PlusPlus,
        // MinusMinus,
        // Assignment,
        // Reference,

        // Static,
        // Return,
        // If,
        // Else,
        // While,
        // For,
        // Enum,
        // Struct,
        // Break,
        // Other,
        // Constant,
    ])
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
