use crate::{
    lexer::token::TokenKind,
    parser::{Parser, expression::Expression, types},
};
pub mod data_parsing;
pub mod identifier_parsing;
pub mod statement_parsing;
use anyhow::{Context, Result};

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

pub fn assignment(parser: &mut Parser, left: Expression, _: i8) -> Result<Expression> {
    let operator = parser.advance().to_owned();
    let value = Box::new(expression(parser, 0)?);
    Ok(Expression::Assignment {
        target: Box::new(left),
        operator,
        value,
        debug_data: parser.debug_data(),
    })
}

pub fn return_expr(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::Return)?;
    let value = expression(parser, 0)?;
    Ok(Expression::Return {
        value: Box::new(value),
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
        let led_function = current_stats.led_function.with_context(|| {
            format!(
                "expected token kind: '{:?}' to have a led function.",
                parser.current().kind
            )
        })?;

        current_expression = led_function(parser, current_expression, current_stats.binding_power)
            .with_context(|| format!("parse expression, led- {:?}", parser.current()))?;
    }
    return Ok(current_expression);
}

pub fn binary(parser: &mut Parser, left: Expression, bp: i8) -> Result<Expression> {
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

pub fn prefix(parser: &mut Parser) -> Result<Expression> {
    let prefix = parser.advance().to_owned();
    let value = expression(parser, 0).context("prefix")?;

    return Ok(Expression::Prefix {
        prefix,
        value: Box::new(value),
        debug_data: parser.debug_data(),
    });
}
pub fn break_expr(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::Break)?;
    return Ok(Expression::Break {
        debug_data: parser.debug_data(),
    });
}
pub fn grouping(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::OpenParen)?;

    let current = parser.current();
    if current.kind == TokenKind::Identifier
        && parser.valid_data_names.contains(current.value.as_str())
    {
        let data_type = types::parse(parser).context("grouping -> TypeConversion -> data_type")?;
        parser.expect(TokenKind::CloseParen)?;
        let value = expression(parser, 0).context("grouping -> TypeConversion -> value")?;

        Ok(Expression::TypeConversion {
            value: Box::new(value),
            data_type,
            debug_data: parser.debug_data(),
        })
    } else {
        let value = Box::new(expression(parser, 0)?);

        parser.expect(TokenKind::CloseParen)?;
        Ok(Expression::Grouping {
            value,
            debug_data: parser.debug_data(),
        })
    }
}
pub fn index(parser: &mut Parser, left: Expression, _: i8) -> Result<Expression> {
    parser.expect(TokenKind::OpenBracket)?;
    let index = expression(parser, 0)?;

    parser.expect(TokenKind::CloseBracket)?;
    Ok(Expression::Index {
        left: Box::new(left),
        index: Box::new(index),
        debug_data: parser.debug_data(),
    })
}

pub fn static_expr(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::Static)?;
    let value = expression(parser, 0)?;
    Ok(Expression::Static {
        value: Box::new(value),
        debug_data: parser.debug_data(),
    })
}
pub fn type_def(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::Typedef)?;
    let data_type = types::parse(parser).context("type_def -> data_type")?;
    let name = parser
        .expect(TokenKind::Identifier)
        .context("type_def -> name")?
        .value;

    Ok(Expression::Typedef {
        data_type,
        name,
        debug_data: parser.debug_data(),
    })
}
