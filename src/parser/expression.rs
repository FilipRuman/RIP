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
pub struct Property {
    pub var_name: String,
    pub var_type: DataType,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Skip,
    DataStructureInitialization {
        values: Vec<Expression>,
        debug_data: DebugData,
    },
    TypeConversion {
        value: Box<Expression>,
        data_type: DataType,
        debug_data: DebugData,
    },
    Typedef {
        data_type: DataType,
        name: String,
        debug_data: DebugData,
    },
    Dereference {
        value: Box<Expression>,
        debug_data: DebugData,
    },
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
    DataTypeAccess {
        data_type: DataType,
        debug_data: DebugData,
    },
    VariableDeclaration {
        var_type: DataType,
        name: String,

        debug_data: DebugData,
    },
    Grouping {
        value: Box<Expression>,
        debug_data: DebugData,
    },
    Struct {
        public: bool,
        name: String,
        properties: Vec<Property>,
        functions: Vec<Expression>,

        debug_data: DebugData,
    },
    NewCodeBlock {
        inside: Vec<Expression>,
        debug_data: DebugData,
    },

    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,

        debug_data: DebugData,
    },
    Function {
        name: String,
        properties: Vec<Property>,
        output: DataType,
        inside: Vec<Expression>,
        debug_data: DebugData,
    },

    MemberExpr {
        left: Box<Expression>,
        right: Box<Expression>,

        debug_data: DebugData,
    },
    AccessReference {
        value: Box<Expression>,

        debug_data: DebugData,
    },
    Break {
        debug_data: DebugData,
    },
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
    Index {
        left: Box<Expression>,
        index: Box<Expression>,

        debug_data: DebugData,
    },

    While {
        condition: Box<Expression>,
        inside: Vec<Expression>,

        debug_data: DebugData,
    },
    Static {
        value: Box<Expression>,
        debug_data: DebugData,
    },

    For {
        iterator_init: Box<Expression>,
        condition: Box<Expression>,
        incr: Box<Expression>,
        inside: Vec<Expression>,

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
            Expression::AccessReference {
                value: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Skip => todo!(),
            Expression::Break { debug_data } => debug_data.to_owned(),
            Expression::Number(_, debug_data) => debug_data.to_owned(),
            Self::Boolean(_, debug_data) => debug_data.to_owned(),
            Expression::String(_, debug_data) => debug_data.to_owned(),
            Expression::Identifier(_, debug_data) => debug_data.to_owned(),
            Expression::Dereference {
                value: _,
                debug_data,
            } => debug_data.to_owned(),
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
            Expression::Grouping {
                value: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Struct {
                public: _,
                name: _,
                properties: _,
                functions: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Binary {
                left: _,
                operator: _,
                right: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::DataStructureInitialization {
                values: _,
                debug_data,
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
            Expression::Index {
                left: _,
                index: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::While {
                condition: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::For {
                iterator_init: _,
                condition: _,
                incr: _,
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::FunctionCall {
                left: _,
                values: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::CompilerData(_, debug_data) => debug_data.to_owned(),
            Expression::Typedef {
                data_type: _,
                name: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::TypeConversion {
                value: _,
                data_type: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::Static {
                value: _,
                debug_data,
            } => debug_data.to_owned(),

            Expression::NewCodeBlock {
                inside: _,
                debug_data,
            } => debug_data.to_owned(),
            Expression::DataTypeAccess {
                data_type: _,
                debug_data,
            } => debug_data.to_owned(),
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
