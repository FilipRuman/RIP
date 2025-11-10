pub mod functions;

use crate::parser::expression::Expression;
use anyhow::{Context, Result};

pub fn generate_code(expressions: Vec<Expression>) -> Result<String> {
    let mut output_code = String::new();

    for expr in expressions {
        output_code +=
            &(expr_to_string(expr).with_context(|| format!("expr: {:?}", expr))? + "{};\n");
    }

    Ok(output_code);
}

pub fn vec_of_expr_to_string(value: Vec<Expression>) -> Result<String> {
    let mut output = String::new();
    for expr in value {
        output += &(expr_to_string(expr).context("for_loop -> inside expressions ")? + ";\n");
    }
    output
}
pub fn expr_to_string(expr: Expression) -> Result<String> {
    match expr {
        Expression::Skip => todo!(),
        Expression::DataStructureInitialization { values, debug_data } => todo!(),
        Expression::TypeConversion {
            value,
            data_type,
            debug_data,
        } => todo!(),
        Expression::Typedef {
            data_type,
            name,
            debug_data,
        } => todo!(),
        Expression::Dereference { value, debug_data } => todo!(),
        Expression::Boolean(_, debug_data) => todo!(),
        Expression::Number(_, debug_data) => todo!(),
        Expression::CompilerData(_, debug_data) => todo!(),
        Expression::String(_, debug_data) => todo!(),
        Expression::Identifier(_, debug_data) => todo!(),
        Expression::Prefix {
            prefix,
            value,
            debug_data,
        } => todo!(),
        Expression::Keyword(token_kind, debug_data) => todo!(),
        Expression::Assignment {
            target,
            operator,
            value,
            debug_data,
        } => todo!(),
        Expression::DataTypeAccess {
            data_type,
            debug_data,
        } => todo!(),
        Expression::VariableDeclaration {
            var_type,
            name,
            debug_data,
        } => todo!(),
        Expression::Grouping { value, debug_data } => todo!(),
        Expression::Struct {
            public,
            name,
            properties,
            functions,
            debug_data,
        } => todo!(),
        Expression::NewCodeBlock { inside, debug_data } => todo!(),
        Expression::Binary {
            left,
            operator,
            right,
            debug_data,
        } => todo!(),
        Expression::Function {
            name,
            properties,
            output,
            inside,
            debug_data,
        } => todo!(),
        Expression::MemberExpr {
            left,
            right,
            debug_data,
        } => todo!(),

        Expression::AccessReference { value, debug_data } => todo!(),
        Expression::Break { debug_data } => todo!(),
        Expression::Return { value, debug_data } => todo!(),
        Expression::If {
            condition,
            inside,
            chained_elses,
            debug_data,
        } => todo!(),
        Expression::Else {
            condition,
            inside,
            debug_data,
        } => todo!(),
        Expression::Index {
            left,
            index,
            debug_data,
        } => todo!(),
        Expression::While {
            condition,
            inside,
            debug_data,
        } => todo!(),
        Expression::Static { value, debug_data } => todo!(),
        Expression::For {
            iterator_init,
            condition,
            incr,
            inside,
            debug_data,
        } => todo!(),
        Expression::FunctionCall {
            left,
            values,
            debug_data,
        } => todo!(),
    }
}
