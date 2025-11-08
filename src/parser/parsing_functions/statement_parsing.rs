use anyhow::{Context, Result};

use crate::{
    lexer::token::TokenKind,
    parser::{
        Parser,
        expression::Expression,
        parsing_functions::{self, expression},
    },
};

pub fn parse_if(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::If)?;
    parser.expect(TokenKind::OpenParen)?;
    let condition = parsing_functions::expression(parser, 0)?;
    parser.expect(TokenKind::CloseParen)?;
    parser.expect(TokenKind::OpenCurly)?;
    let mut inside = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        inside.push(parsing_functions::expression(parser, 0)?);
        parser
            .expect(TokenKind::SemiColon)
            .context("expected to find a semicolon after a expression - function contents")?;
    }
    parser.expect(TokenKind::CloseCurly)?;

    let mut chained_elses = Vec::new();

    while parser.current().kind == TokenKind::Else {
        let (else_value, break_else) = parse_else(parser)?;
        chained_elses.push(else_value);
        if break_else {
            break;
        }
    }

    Ok(Expression::If {
        condition: Box::new(condition),
        inside,
        chained_elses,
        debug_data: parser.debug_data(),
    })
}
fn parse_else(parser: &mut Parser) -> Result<(Expression, bool)> {
    parser.expect(TokenKind::Else)?;
    let (break_else, condition) = if parser.current().kind == TokenKind::If {
        parser.expect(TokenKind::If)?;
        parser.expect(TokenKind::OpenParen)?;
        let condition = parsing_functions::expression(parser, 0)?;
        parser.expect(TokenKind::CloseParen)?;
        (false, Some(Box::new(condition)))
    } else {
        (true, None)
    };
    parser.expect(TokenKind::OpenCurly)?;

    let mut inside = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        inside.push(parsing_functions::expression(parser, 0)?);
        parser
            .expect(TokenKind::SemiColon)
            .context("expected to find a semicolon after a expression - function contents")?;
    }
    parser.expect(TokenKind::CloseCurly)?;
    Ok((
        Expression::Else {
            condition,
            inside,
            debug_data: parser.debug_data(),
        },
        break_else,
    ))
}

pub fn parse_while(parser: &mut Parser) -> Result<Expression> {
    parser.expect(TokenKind::If)?;
    parser.expect(TokenKind::OpenParen)?;
    let condition = parsing_functions::expression(parser, 0)?;
    parser.expect(TokenKind::CloseParen)?;
    parser.expect(TokenKind::OpenCurly)?;
    let mut inside = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        inside.push(parsing_functions::expression(parser, 0)?);
        parser
            .expect(TokenKind::SemiColon)
            .context("expected to find a semicolon after a expression - function contents")?;
    }
    parser.expect(TokenKind::CloseCurly)?;

    Ok(Expression::While {
        condition: Box::new(condition),
        inside,
        debug_data: parser.debug_data(),
    })
}

pub fn parse_for(parser: &mut Parser) -> Result<Expression> {
    // for(int i =0;i<25;i++){
    // ...
    // }
    parser.expect(TokenKind::For)?;
    parser.expect(TokenKind::OpenParen)?;
    let iterator_init = Box::new(parsing_functions::expression(parser, 0)?);
    parser.expect(TokenKind::SemiColon)?;
    let condition = Box::new(parsing_functions::expression(parser, 0)?);
    parser.expect(TokenKind::SemiColon)?;
    let incr = Box::new(parsing_functions::expression(parser, 0)?);
    parser.expect(TokenKind::CloseParen)?;

    parser.expect(TokenKind::OpenCurly)?;
    let mut inside = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        inside.push(parsing_functions::expression(parser, 0)?);
        parser
            .expect(TokenKind::SemiColon)
            .context("expected to find a semicolon after a expression - function contents")?;
    }
    parser.expect(TokenKind::CloseCurly)?;

    Ok(Expression::For {
        iterator_init,
        condition,
        incr,
        inside,
        debug_data: parser.debug_data(),
    })
}
