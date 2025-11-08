use crate::parser::{Parser, expression::Expression};
use anyhow::{Result, bail};

pub fn string(parser: &mut Parser) -> Result<Expression> {
    let token = parser.advance();

    return Ok(Expression::String(
        token.value.to_owned(),
        parser.debug_data(),
    ));
}

pub fn number(parser: &mut Parser) -> Result<Expression> {
    let token = parser.advance();

    return Ok(Expression::Number(
        str_to_num(&token.value)?,
        parser.debug_data(),
    ));
}
pub fn str_to_num(s: &str) -> Result<u32, std::num::ParseIntError> {
    if let Some(hex) = s.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)
    } else if let Some(bin) = s.strip_prefix("0b") {
        u32::from_str_radix(bin, 2)
    } else if let Some(oct) = s.strip_prefix("0o") {
        u32::from_str_radix(oct, 8)
    } else {
        // default to decimal
        s.parse::<u32>()
    }
}

pub fn boolean(parser: &mut Parser) -> Result<Expression> {
    let token = parser.advance();
    return Ok(Expression::Boolean(
        match token.kind {
            crate::lexer::token::TokenKind::True => true,
            crate::lexer::token::TokenKind::False => false,
            _ => {
                bail!(
                    "expected to find token of kind either True or False, found: '{:?}'",
                    token.kind
                )
            }
        },
        parser.debug_data(),
    ));
}
