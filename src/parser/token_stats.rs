use std::collections::HashMap;

use crate::{
    lexer::token::TokenKind,
    parser::{
        LedFunction, NodFunction,
        parsing_functions::{self, identifier_parsing, type_def},
    },
};

pub struct TokenStats {
    pub binding_power: i8,
    pub nod_function: Option<NodFunction>,
    pub led_function: Option<LedFunction>,
}
pub fn token_stats() -> HashMap<TokenKind, TokenStats> {
    HashMap::from([
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
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::BitwiseShiftRight,
            TokenStats {
                binding_power: 2,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Plus,
            TokenStats {
                binding_power: 2,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Minus,
            TokenStats {
                binding_power: 2,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Star,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Slash,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Percent,
            TokenStats {
                binding_power: 3,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Equals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::NotEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Less,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::LessEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Greater,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::GreaterEquals,
            TokenStats {
                binding_power: 4,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Or,
            TokenStats {
                binding_power: 1,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::And,
            TokenStats {
                binding_power: 1,
                nod_function: None,
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Not,
            TokenStats {
                binding_power: 1,
                nod_function: Some(parsing_functions::prefix),
                led_function: Some(parsing_functions::binary),
            },
        ),
        (
            TokenKind::Number,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::data_parsing::number),
                led_function: None,
            },
        ),
        (
            TokenKind::String,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::data_parsing::string),
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
                nod_function: Some(parsing_functions::data_parsing::boolean),
                led_function: None,
            },
        ),
        (
            TokenKind::False,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::data_parsing::boolean),
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
        (
            TokenKind::Typedef,
            TokenStats {
                binding_power: 0,
                nod_function: Some(type_def),
                led_function: None,
            },
        ),
        (
            TokenKind::PlusEquals,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::MinusEquals,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::StarEquals,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::SlashEquals,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::PlusPlus,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::MinusMinus,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::Equals,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::Reference,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::assignment),
            },
        ),
        (
            TokenKind::Static,
            TokenStats {
                binding_power: 0,
                nod_function: Some(parsing_functions::static_expr),
                led_function: None,
            },
        ),
        (
            TokenKind::OpenBracket,
            TokenStats {
                binding_power: 5,
                nod_function: None,
                led_function: Some(parsing_functions::index),
            },
        ),
        (
            TokenKind::CloseBracket,
            TokenStats {
                binding_power: 0,
                nod_function: None,
                led_function: None,
            },
        ),
        // Dot,
        // Colon,
        // Question,
        // Reference,
        // Other,
        // Constant,
    ])
}
