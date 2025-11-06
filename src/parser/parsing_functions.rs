use crate::{
    lexer::token::TokenKind,
    parser::{Parser, expression::Expression},
};
use anyhow::{Context, Result, bail};
use log::info;

pub fn function_call(parser: &mut Parser, left: Expression, _: i8) -> Result<Expression> {
    parser.expect(TokenKind::OpenParen)?;
    let mut output = Vec::new();
    loop {
        output.push(expression(parser, 0).with_context(|| {
            format!("function call- parse input values-> {:?}", parser.current())
        })?);

        if parser.advance().kind == TokenKind::CloseParen {
            break;
        }
    }

    Ok(Expression::FunctionCall {
        left: Box::new(left),
        values: output,
        debug_data: parser.debug_data(),
    })
}
pub fn expression(parser: &mut Parser, bp: i8) -> Result<Expression> {
    let mut current_expression = {
        let nod_function = parser.current_stats()?.nod_function.with_context(|| {
            format!(
                "expected token kind: '{:?}' to have nod function.",
                parser.current().kind
            )
        })?;
        let current = parser.current().to_owned();
        nod_function(parser).with_context(|| format!("parse expression, nod- {:?}", current))?
    };

    while let current_stats = parser.current_stats()?
        && current_stats.binding_power > bp
    {
        current_expression = {
            let led_function = current_stats.led_function.with_context(|| {
                format!(
                    "expected token kind: '{:?}' to have a led function.",
                    parser.current().kind
                )
            })?;
            led_function(parser, current_expression, current_stats.binding_power)
                .with_context(|| format!("parse expression, led- {:?}", parser.current()))?
        }
    }
    return Ok(current_expression);
}

pub fn return_expr(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::Return)?;
    let value = expression(parser, 0)?;
    Ok(Expression::Return {
        value: Box::new(value),
        debug_data: parser.debug_data(),
    })
}

pub fn binary_expression(parser: &mut Parser, left: Expression, bp: i8) -> Result<Expression> {
    let operator = parser.advance().to_owned();
    let right =
        expression(parser, bp).context("binary operation- parse expression on the right")?;

    Ok(Expression::Binary {
        left: Box::new(left),
        operator: operator,
        right: Box::new(right),
        debug_data: parser.debug_data(),
    })
}

pub fn compiler_data(parser: &mut Parser) -> Result<Expression> {
    Ok(Expression::CompilerData(
        parser.advance().value.to_owned(),
        parser.debug_data(),
    ))
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

pub fn prefix(parser: &mut Parser) -> Result<Expression> {
    let prefix = parser.advance().to_owned();
    let value = expression(parser, 0).context("prefix")?;

    return Ok(Expression::Prefix {
        prefix,
        value: Box::new(value),
        debug_data: parser.debug_data(),
    });
}

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
fn str_to_num(s: &str) -> Result<u32, std::num::ParseIntError> {
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
