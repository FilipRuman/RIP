use crate::lexer::token::Token;
use crate::parser::expression::Property;
use crate::parser::{Parser, parsing_functions};
use crate::{lexer::token::TokenKind, parser::parsing_functions::data_parsing::str_to_num};
use anyhow::{Context, Result, bail};

#[derive(Debug, Clone)]
pub struct EnumField {
    name: String,
    value: u32,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Array { length: u32, inside: Box<DataType> },
    Data { unsigned: bool },
    Struct { properties: Vec<Property> },
    Enum { fields: Vec<EnumField> },
    Pointer(Box<DataType>),
}

pub fn parse(parser: &mut Parser) -> Result<DataType> {
    let unsigned = {
        let current = parser.current();

        let unsigned = current.value == "unsigned";
        if unsigned {
            parser.advance();
        }
        unsigned
    };

    let current = parser.advance().to_owned();
    match current.kind {
        TokenKind::Identifier => {
            identifier_type(parser, unsigned, current).context("types::parse -> identifier")
        }
        TokenKind::Enum => enum_type(parser).context("types::parse -> Enum"),
        TokenKind::Struct => struct_type(parser).context("types::parse -> Struct"),
        other => {
            bail!(
                "types::parse: expected to fine 'Identifier' || 'Enum' || 'Struct', found: {:?} -> parsing of this token kind as datatype is not supported",
                other
            )
        }
    }
}
pub fn wrap_data_type_in_an_array(
    mut data_type: DataType,
    parser: &mut Parser,
) -> Result<DataType> {
    while parser.current().kind == TokenKind::OpenBracket {
        parser.expect(TokenKind::OpenBracket)?;
        let length =
            parsing_functions::data_parsing::str_to_num(&parser.expect(TokenKind::Number)?.value)?;
        parser.expect(TokenKind::CloseBracket)?;
        data_type = DataType::Array {
            length,
            inside: Box::new(data_type),
        };
    }

    Ok(data_type)
}
fn identifier_type(parser: &mut Parser, unsigned: bool, current: Token) -> Result<DataType> {
    let mut output = DataType::Data { unsigned };
    while current.kind == TokenKind::Star {
        output = DataType::Pointer(Box::new(output));
        parser.advance();
    }
    Ok(output)
}

fn enum_type(parser: &mut Parser) -> Result<DataType> {
    parser.expect(TokenKind::OpenCurly)?;
    let mut current_value = 0;
    let mut fields = Vec::new();
    let mut end = false;
    while !end {
        let field_name = parser.expect(TokenKind::Identifier)?.value;
        match parser.advance().kind {
            TokenKind::Equals => {
                current_value = str_to_num(&parser.advance().value)?;
                end = parser.advance().kind == TokenKind::CloseCurly;
            }
            TokenKind::Comma => {}
            TokenKind::CloseCurly => {
                end = true;
            }
            kind => {
                bail!(
                    "expected to find token of kind: 'Comma' || 'Assignment' || 'CloseParen', found: '{kind:?}'"
                )
            }
        }

        fields.push(EnumField {
            name: field_name,
            value: current_value,
        });

        current_value += 1;
    }
    Ok(DataType::Enum { fields })
}

fn struct_type(parser: &mut Parser) -> Result<DataType> {
    parser.expect(TokenKind::OpenCurly)?;
    let mut properties = Vec::new();
    while parser.current().kind != TokenKind::CloseCurly {
        let data_type = parse(parser)?;
        let name = parser.expect(TokenKind::Identifier)?.value;
        parser
            .expect(TokenKind::SemiColon)
            .context("expected to find a semicolon after a expression - struct contents")?;

        properties.push(Property {
            var_name: name,
            var_type: data_type,
        });
    }

    parser.expect(TokenKind::CloseCurly)?;
    Ok(DataType::Struct { properties })
}
