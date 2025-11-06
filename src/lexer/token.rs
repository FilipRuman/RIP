#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub line: u16, // needed for debugging
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Tab,
    Comment,
    CompilerData,

    WhiteSpace,
    EndOfFile,
    NextLine,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Comma,
    Dot,
    SemiColon,
    Colon,
    Arrow,
    Question,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,

    PlusPlus,
    MinusMinus,

    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    Not,
    And,
    Or,

    BitwiseShiftLeft,
    BitwiseShiftRight,

    Assignment,
    Reference,

    Number,
    String,
    True,
    False,

    Identifier,
    Static,
    Return,
    If,
    Else,
    While,
    For,
    Enum,
    Struct,
    Break,
    Other,
    Constant,
    Typedef,
}
