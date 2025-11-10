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
    Increment {
        target: Box<Expression>,
        debug_data: DebugData,
    },
    Decrement {
        target: Box<Expression>,
        debug_data: DebugData,
    },
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
