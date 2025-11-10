use crate::{
    code_gen::{expr_to_string, vec_of_expr_to_string},
    lexer::token::Token,
    parser::expression::Expression,
};
use anyhow::Result;
use log::info;

pub fn assignment(target: Expression, operator: Token, value: Expression) -> Result<String> {
    Ok(format!(
        "{} {} {}",
        expr_to_string(target)?,
        operator.kind.to_str(),
        expr_to_string(value)?
    ))
}

pub fn new_code_block(inside: Vec<Expression>) -> Result<String> {
    Ok(format!("{{\n{}}}", vec_of_expr_to_string(inside)?))
}

pub fn for_loop(
    iterator_init: Expression,
    condition: Expression,
    incr: Expression,
    inside: Vec<Expression>,
) -> Result<String> {
    Ok(format!(
        r"
// for
{} // iterator init
while ({}) {{ // condition
//inside
{}  
{} // increment
}}
",
        expr_to_string(iterator_init)?,
        expr_to_string(condition)?,
        vec_of_expr_to_string(inside)?,
        expr_to_string(incr)?,
    ))
}
pub fn while_loop(condition: Expression, inside: Vec<Expression>) -> Result<String> {
    Ok(format!(
        "while ({}){{\n{}}}",
        expr_to_string(condition)?,
        vec_of_expr_to_string(inside)?
    ))
}
pub fn boolean(value: bool) -> String {
    value.to_string()
}

pub fn identifier(value: String) -> String {
    // replace some know c identifiers names to zing ones
    // or do some other operations
    value
}

pub fn string(value: String) -> String {
    format!("\"{}\"", value)
}
