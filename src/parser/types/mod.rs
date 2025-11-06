use crate::lexer::token::TokenKind;
use crate::parser::Parser;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum DataType {
    Data(String),
    Pointer(Box<DataType>),
}

pub fn parse(parser: &mut Parser) -> Result<DataType> {
    let mut current = parser.current().clone();

    let unsigned = current.value == "unsigned";
    if unsigned {
        parser.advance();
    }

    current = parser.expect(TokenKind::Identifier)?.clone();
    let mut output = DataType::Data(current.value);
    while current.kind == TokenKind::Star {
        output = DataType::Pointer(Box::new(output));
        parser.advance();
    }
    Ok(output)
}
