use crate::lexer::{
    Lexer,
    token::{Token, TokenKind},
    tokenization_function::{
        handle_comments, handle_compiler_data, handle_identifier, handle_number, handle_string,
    },
};
use anyhow::{Context, Result, bail};
use std::collections::HashMap;

type TokenizationFunc = fn(u16, &mut Lexer) -> Result<Token>;

#[derive(Clone, Copy)]
pub enum TokenPattern {
    Fast {
        kind: TokenKind,
        use_second_char: bool,
    },
    Long(TokenizationFunc),
}

struct TokenPatternInitialization {
    start_chars: Vec<char>,
    second_char: char,
    pattern: TokenPattern,
}

impl TokenPatternInitialization {
    fn new(start_chars: Vec<char>, second_char: char, pattern: TokenPattern) -> Self {
        Self {
            start_chars,
            second_char,
            pattern,
        }
    }
}

pub fn setup_token_patters() -> Result<HashMap<(char, char), TokenPattern>> {
    let patterns = patterns();
    let mut hashmap = HashMap::new();

    let mut i = 0;
    for pat in patterns {
        for start_char in pat.start_chars {
            let key = (start_char, pat.second_char);
            if hashmap.contains_key(&key) {
                bail!(
                    "there is another 'token kind' that has pattern with the same char combination: '{}' '{}', pattern index: {i}",
                    start_char,
                    pat.second_char
                );
            }
            hashmap.insert(key, pat.pattern);
        }
        i += 1;
    }

    Ok(hashmap)
}

///INFO: Uses Lexer to get current and next char and get right function for parsing value that starts
/// with those chars
pub fn pattern_for_current_char(lexer: &mut Lexer) -> Option<TokenPattern> {
    let current = lexer.current();
    let next = lexer.next();
    match lexer.token_patterns.get(&(current, next)) {
        Some(val) => Some(*val),
        // if there is no pattern with next char, then there might be one without it
        None => match lexer.token_patterns.get(&(current, ' ')) {
            Some(val) => Some(*val),
            None => None,
        },
    }
}

const SYMBOL_CHARS: [char; 53] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn patterns() -> Vec<TokenPatternInitialization> {
    let patterns: Vec<TokenPatternInitialization> = vec![
        TokenPatternInitialization::new(vec!['/'], '/', TokenPattern::Long(handle_comments)),
        TokenPatternInitialization::new(vec!['"'], ' ', TokenPattern::Long(handle_string)),
        TokenPatternInitialization::new(vec!['#'], ' ', TokenPattern::Long(handle_compiler_data)),
        TokenPatternInitialization::new(NUMBERS.to_vec(), ' ', TokenPattern::Long(handle_number)),
        TokenPatternInitialization::new(
            SYMBOL_CHARS.to_vec(),
            ' ',
            TokenPattern::Long(handle_identifier),
        ),
        TokenPatternInitialization::new(
            vec!['\t'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Tab,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![' '],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::WhiteSpace,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['\0'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::EndOfFile,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['\n'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::NextLine,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['('],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::OpenParen,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![')'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::CloseParen,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['['],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::OpenBracket,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![']'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::CloseBracket,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['{'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::OpenCurly,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['}'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::CloseCurly,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![','],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Comma,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['.'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Dot,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![';'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::SemiColon,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec![':'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Colon,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['-'],
            '>',
            TokenPattern::Fast {
                kind: TokenKind::Arrow,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['?'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Question,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['+'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::PlusEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['+'],
            '+',
            TokenPattern::Fast {
                kind: TokenKind::PlusPlus,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['+'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Plus,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['-'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::MinusEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['-'],
            '-',
            TokenPattern::Fast {
                kind: TokenKind::MinusMinus,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['-'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Minus,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['*'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::StarEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['*'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Star,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['/'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::SlashEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['/'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Slash,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['%'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Percent,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['='],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::Equals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['!'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::NotEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['<'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::LessEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['<'],
            '<',
            TokenPattern::Fast {
                kind: TokenKind::BitwiseShiftLeft,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['<'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Less,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['>'],
            '=',
            TokenPattern::Fast {
                kind: TokenKind::GreaterEquals,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['>'],
            '>',
            TokenPattern::Fast {
                kind: TokenKind::BitwiseShiftRight,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['>'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Greater,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['!'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Not,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['&'],
            '&',
            TokenPattern::Fast {
                kind: TokenKind::And,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['|'],
            '|',
            TokenPattern::Fast {
                kind: TokenKind::Or,
                use_second_char: true,
            },
        ),
        TokenPatternInitialization::new(
            vec!['='],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Assignment,
                use_second_char: false,
            },
        ),
        TokenPatternInitialization::new(
            vec!['&'],
            ' ',
            TokenPattern::Fast {
                kind: TokenKind::Reference,
                use_second_char: false,
            },
        ),
    ];
    patterns
}
