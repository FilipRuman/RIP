use crate::{
    lexer::token::{Token, TokenKind},
    parser::types::DataType,
};

#[derive(Debug, Clone)]
pub struct DebugData {
    pub line: u16,
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct FunctionProperty {
    pub var_name: String,
    pub var_type: DataType,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Skip,
    As(Box<Expression>, DataType, DebugData),
    Dereference(Box<Expression>, DebugData),
    Reference(Box<Expression>, DebugData),
    Boolean(bool, DebugData),

    Number(u32, DebugData),

    CompilerData(String, DebugData),
    String(String, DebugData),
    Identifier(String, DebugData),
    Prefix {
        prefix: Token,
        value: Box<Expression>,
        debug_data: DebugData,
    },
    Keyword(TokenKind, DebugData),
    // target operator value
    Assignment {
        target: Box<Expression>,
        operator: Token,
        value: Box<Expression>,

        debug_data: DebugData,
    },
    // type name mutable
    VariableDeclaration {
        var_type: DataType,
        name: String,

        debug_data: DebugData,
    },
    Grouping(Box<Expression>, DebugData),
    Struct {
        public: bool,
        name: String,
        properties: Vec<Expression>,
        functions: Vec<Expression>,

        debug_data: DebugData,
    },
    StructProperty {
        var_name: String,
        var_type: DataType,

        debug_data: DebugData,
    },
    StructFunction {
        name: String, /* ,type : Type */

        debug_data: DebugData,
    },

    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,

        debug_data: DebugData,
    },
    ClassInstantiation {
        name: String,
        properties: Vec<Expression>,

        debug_data: DebugData,
    },
    ArrayInitialization {
        length: u32,
        properties: Vec<Expression>,
        debug_data: DebugData,
        inside_type: DataType,
    },
    Function {
        name: String,
        properties: Vec<FunctionProperty>,
        output: DataType,
        inside: Vec<Expression>,
        debug_data: DebugData,
    },

    MemberExpr {
        left: Box<Expression>,
        right: Box<Expression>,

        debug_data: DebugData,
    },
    Break,
    Return {
        value: Box<Expression>,

        debug_data: DebugData,
    },
    If {
        condition: Box<Expression>,
        inside: Vec<Expression>,

        /// those will be else statements with or without conditions
        chained_elses: Vec<Expression>,
        debug_data: DebugData,
    },
    Else {
        condition: Option<Box<Expression>>,
        inside: Vec<Expression>,

        debug_data: DebugData,
    },
    SquareBrackets {
        left: Box<Expression>,
        indexes: Vec<Expression>,

        debug_data: DebugData,
    },

    While {
        condition: Box<Expression>,
        inside: Vec<Expression>,

        debug_data: DebugData,
    },
    For {
        iterator_name: String,
        iteration_target: Box<Expression>,
        inside: Vec<Expression>,

        debug_data: DebugData,
    },
    Range {
        from: Box<Expression>,
        to: Box<Expression>,
        debug_data: DebugData,
    },
    FunctionCall {
        left: Box<Expression>,
        values: Vec<Expression>,
        debug_data: DebugData,
    },
}
impl Expression {
    pub fn debug_data(&self) -> DebugData {
        match self {
            Expression::Skip => todo!(),
            Expression::Break => todo!(),
            Expression::Number(_, debug_data) => debug_data.to_owned(),
            Self::Boolean(_, debug_data) => debug_data.to_owned(),
            Expression::String(_, debug_data) => debug_data.to_owned(),
            Expression::Identifier(_, debug_data) => debug_data.to_owned(),
            Expression::Dereference(_, debug_data) => debug_data.to_owned(),
            Expression::Prefix {
                prefix: _,
                value: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Keyword(_, debug_data) => debug_data.to_owned(),
            Expression::Assignment {
                target: _,
                operator: _,
                value: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::VariableDeclaration {
                var_type: _,
                name: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Grouping(_, debug_data) => debug_data.to_owned(),
            Expression::Struct {
                public: _,
                name: _,
                properties: _,
                functions: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::StructProperty {
                var_name: _,
                var_type: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::StructFunction {
                name: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Binary {
                left: _,
                operator: _,
                right: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::ClassInstantiation {
                name: _,
                properties: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::ArrayInitialization {
                properties: _,
                debug_data,
                length: _,
                inside_type: _,
            } => debug_data.to_owned(),
            Expression::Function {
                name: _,
                properties: _,
                output: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::MemberExpr {
                left: _,
                right: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Return {
                value: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::If {
                condition: _,
                inside: _,
                debug_data,
                chained_elses: _,
            } => debug_data.to_owned(),
            Expression::Else {
                condition: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::SquareBrackets {
                left: _,
                indexes: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::While {
                condition: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::For {
                iterator_name: _,
                iteration_target: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Range {
                from: _,
                to: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::FunctionCall {
                left: _,
                values: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Reference(_, debug_data) => debug_data.to_owned(),
            Expression::As(_, _, debug_data) => debug_data.to_owned(),
            Expression::CompilerData(_, debug_data) => debug_data.to_owned(),
        }
    }
}

const SHOW_EXPRESSION_DEBUG: bool = true;
pub fn debug_expression(text: &str) {
    if !SHOW_EXPRESSION_DEBUG {
        return;
    }

    println!("{}", text);
}
