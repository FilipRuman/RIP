use crate::{
    lexer::token::TokenKind,
    parse,
    parser::{
        Parser,
        expression::{Expression, Property},
        parsing_functions::{self},
        types::{self, DataType},
    },
};

use anyhow::{Context, Result};
pub fn identifier(parser: &mut Parser) -> Result<Expression> {
    let first = parser.current().to_owned();

    if parser.valid_data_names.contains(&first.value) {
        handle_function_or_variable_declaration(parser)
            .with_context(|| format!("identifier - data type name: {}", first.value.as_str()))
    } else {
        parser.expect(first.kind)?;
        Ok(Expression::Identifier(first.value, parser.debug_data()))
    }
}

fn handle_function_or_variable_declaration(parser: &mut Parser) -> Result<Expression> {
    let data_type = types::parse(parser)
        .context("parse data type for: handle_function_or_variable_declaration")?;
    if parser.current().kind != TokenKind::Identifier {
        return Ok(Expression::DataTypeAccess {
            data_type: data_type,
            debug_data: parser.debug_data(),
        });
    }

    let name = parser.advance().to_owned();

    if parser.current().kind == TokenKind::OpenParen {
        handle_function_declaration(data_type, name.value, parser)
            .context("handle_function_declaration")
    } else {
        Ok(Expression::VariableDeclaration {
            var_type: data_type,
            name: name.value,
            debug_data: parser.debug_data(),
        })
    }
}
fn handle_function_declaration(
    output_data_type: DataType,
    name: String,
    parser: &mut Parser,
) -> Result<Expression> {
    parser.expect(TokenKind::OpenParen)?;

    let mut properties = Vec::new();

    if parser.current().kind != TokenKind::CloseParen {
        loop {
            let data_type = types::parse(parser).context("parse function input data types")?;
            let name = parser.expect(TokenKind::Identifier)?.value.to_owned();
            properties.push(Property {
                var_name: name,
                var_type: data_type,
            });
            if parser.current().kind == TokenKind::CloseParen {
                break;
            }
            parser
                .expect(TokenKind::Comma)
                .context("function properties")?;
        }
    }
    parser.expect(TokenKind::CloseParen)?;
    parser.expect(TokenKind::OpenCurly)?;

    let mut inside = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        inside.push(parsing_functions::expression(parser, 0).context("inside function")?);
        if parser.current().kind == TokenKind::SemiColon {
            parser.advance();
        }
    }

    parser.expect(TokenKind::CloseCurly)?;
    Ok(Expression::Function {
        name,
        properties,
        output: output_data_type,
        inside: inside,
        debug_data: parser.debug_data(),
    })
}
