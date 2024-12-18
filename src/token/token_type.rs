use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum RawToken {
    // ASCII identifiers (including underscores)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    IdentifierAscii,

    // Unicode identifiers (including underscores)
    #[regex(r"[\p{Letter}\p{Mark}_][\p{Letter}\p{Mark}\p{Number}_]*", priority = 1)]
    IdentifierUnicode,

    // Numbers: integers, floats, scientific notation, complex numbers
    #[regex(r"(\d*\.\d+|\d+\.|\d+)([eE][+-]?\d+)?[if]*", priority = 4)]
    Number,

    // Binary numbers (e.g., #b1010, #b1101)
    #[regex(r"##[01]+", priority = 2)]
    Binary,

    // Hexadecimal numbers (e.g., #ff, #7f), excluding #b
    #[regex(r"#([0-9a-fA-F]+)", priority = 3)]
    Hexadecimal,

    // Octal numbers (e.g., #o23, #o24)
    #[regex(r"#o([0-7]+)", priority = 2)]
    Octal,

    // Whitespace (including Unicode spaces)
    #[regex(
        r"[ \t\n\f\u{00A0}\u{1680}\u{2000}-\u{200A}\u{202F}\u{205F}\u{3000}]+",
        logos::skip
    )]
    Whitespace,

    #[regex(r"//[^\n]*")] // Skip inline comments
    SingleLineComment,
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")] // Skip multi-line comments
    MultiLineComment,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("<")]
    LESS,
    #[token(">")]
    GREATER,
    #[token("!")]
    NOT,
    #[token("^")]
    XOR,
    #[token("%")]
    PERCENT,
    #[token("|")]
    OR,
    #[token("&")]
    AND,
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("+=")]
    PlusEqual,
    #[token("-?")]
    MinusEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    Greaterequal,
    #[token("!=")]
    NotEqual,
    #[token("^=")]
    XorEqual,
    #[token("%=")]
    PercentEqual,

    /*#[token("%")]
    PERCENT,*/
    #[token("||")]
    OrOr,
    #[token("&&")]
    AndAnd,

    // Parentheses
    #[token("(")]
    OpenParentesis,
    #[token(")")]
    CloseParentesis,

    // Square brackets
    #[token("[")]
    OpenSQParentesis,
    #[token("]")]
    CloseSQParentesis,

    // Curly brackets
    #[token("{")]
    OpenCurParentesis,
    #[token("}")]
    CloseCurParentesis,

    #[regex(r"true|false", priority = 5)]
    BOOLEAN,

    // Strings: Matches double-quoted strings, including escape sequences
    #[regex(r#""([^"\\]|\\.)*""#)]
    STRING,

    // Characters: Matches single-quoted characters, including escape sequences
    #[regex(r#"'([^'\\]|\\.)'"#)]
    CHAR,

    // Single dot
    #[token(".")]
    Dot,

    // Type tokens
    #[token("i8")]
    TYPEI8,
    #[token("i16")]
    TYPEI16,
    #[token("i32")]
    TYPEI32,
    #[token("i64")]
    TYPEI64,
    #[token("u8")]
    TYPEU8,
    #[token("u16")]
    TYPEU16,
    #[token("u32")]
    TYPEU32,
    #[token("u64")]
    TYPEU64,
    #[token("f32")]
    TYPEF32,
    #[token("f64")]
    TYPEF64,
    #[token("c32")]
    TYPEC32,
    #[token("c64")]
    TYPEC64,
    #[token("char")]
    TYPECHAR,
    #[token("string")]
    TYPESTRING,
    #[token("bool")]
    TYPEBOOL,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Integer,
    Double,
    Boolean,
    Plus,
    Minus,
    Not,
    Star,
    Divide,
    Xor,
    Percent,
    Or,
    And,
    Equal,
    Less,
    Greater,
    PlusPlus,
    MinusMinus,
    PlusEqual,
    MinusEqual,
    NotEqual,
    StarEqual,
    DivideEqual,
    XorEqual,
    PercentEqual,
    OrOr,
    AndAnd,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Dot,
    Identifier,
    Char,
    String,
    KMain,
    KVar,
    KIf,
    KWhile,
    KElse,
    KFor,
    KBreak,
    KFun,
    KReturn,
    KNullptr,
    OpenParenthesis,
    OpenSqParenthesis,
    OpenCurParenthesis,
    CloseParenthesis,
    CloseSqParenthesis,
    CloseCurParenthesis,
    Comma,
    Colon,
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeC32,
    TypeC64,
    TypeChar,
    TypeString,
    TypeBool,
    Comment,
    Unknown,
    Eoft,
}

#[allow(dead_code)]
impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::KMain
                | TokenType::KVar
                | TokenType::KIf
                | TokenType::KWhile
                | TokenType::KElse
                | TokenType::KFor
                | TokenType::KBreak
                | TokenType::KFun
                | TokenType::KReturn
        )
    }

    pub fn map_keword_to_token_type(keyword: &str) -> TokenType {
        match keyword {
            "main" => TokenType::KMain,
            "var" => TokenType::KVar,
            "if" => TokenType::KIf,
            "while" => TokenType::KWhile,
            "else" => TokenType::KElse,
            "for" => TokenType::KFor,
            "break" => TokenType::KBreak,
            "fun" => TokenType::KFun,
            "return" => TokenType::KReturn,
            _ => TokenType::Identifier,
        }
    }

    pub fn map_raw_token_type(raw_token: RawToken, raw_val: &str) -> (TokenType, &str) {
        match raw_token {
            RawToken::IdentifierAscii => (TokenType::map_keword_to_token_type(raw_val), ""),
            RawToken::IdentifierUnicode => (TokenType::Identifier, ""),
            RawToken::Binary => (TokenType::Integer, ""),
            RawToken::Hexadecimal => (TokenType::Integer, ""),
            RawToken::Octal => (TokenType::Integer, ""),
            RawToken::Whitespace => (TokenType::Unknown, ""),
            RawToken::SingleLineComment => (TokenType::Comment, ""),
            RawToken::MultiLineComment => (TokenType::Comment, ""),
            RawToken::Plus => (TokenType::Plus, ""),
            RawToken::Minus => (TokenType::Minus, ""),
            RawToken::Star => (TokenType::Star, ""),
            RawToken::Slash => (TokenType::Divide, ""),
            RawToken::LESS => (TokenType::Less, ""),
            RawToken::GREATER => (TokenType::Greater, ""),
            RawToken::NOT => (TokenType::Not, ""),
            RawToken::XOR => (TokenType::Xor, ""),
            RawToken::PERCENT => (TokenType::Percent, ""),
            RawToken::OR => (TokenType::Or, ""),
            RawToken::AND => (TokenType::And, ""),
            RawToken::Equal => (TokenType::Equal, ""),
            RawToken::Colon => (TokenType::Colon, ""),
            RawToken::Comma => (TokenType::Comma, ""),
            RawToken::PlusPlus => (TokenType::PlusPlus, ""),
            RawToken::MinusMinus => (TokenType::MinusMinus, ""),
            RawToken::PlusEqual => (TokenType::PlusEqual, ""),
            RawToken::MinusEqual => (TokenType::MinusEqual, ""),
            RawToken::LessEqual => (TokenType::LessEqual, ""),
            RawToken::Greaterequal => (TokenType::GreaterEqual, ""),
            RawToken::NotEqual => (TokenType::NotEqual, ""),
            RawToken::XorEqual => (TokenType::XorEqual, ""),
            RawToken::PercentEqual => (TokenType::PercentEqual, ""),
            RawToken::OrOr => (TokenType::OrOr, ""),
            RawToken::AndAnd => (TokenType::AndAnd, ""),
            RawToken::OpenParentesis => (TokenType::OpenParenthesis, ""),
            RawToken::CloseParentesis => (TokenType::CloseParenthesis, ""),
            RawToken::OpenSQParentesis => (TokenType::OpenSqParenthesis, ""),
            RawToken::CloseSQParentesis => (TokenType::CloseSqParenthesis, ""),
            RawToken::OpenCurParentesis => (TokenType::OpenCurParenthesis, ""),
            RawToken::CloseCurParentesis => (TokenType::CloseCurParenthesis, ""),
            RawToken::BOOLEAN => (TokenType::Boolean, ""),
            RawToken::STRING => (TokenType::String, 
                raw_val.trim_start_matches('"').trim_end_matches('"')),
            RawToken::CHAR => (TokenType::Char, 
                raw_val.trim_start_matches('\'').trim_end_matches('\'')),
            RawToken::Dot => (TokenType::Dot, ""),
            RawToken::TYPEI8 => (TokenType::TypeI8, ""),
            RawToken::TYPEI16 => (TokenType::TypeI16, ""),
            RawToken::TYPEI32 => (TokenType::TypeI32, ""),
            RawToken::TYPEI64 => (TokenType::TypeI64, ""),
            RawToken::TYPEU8 => (TokenType::TypeU8, ""),
            RawToken::TYPEU16 => (TokenType::TypeU16, ""),
            RawToken::TYPEU32 => (TokenType::TypeU32, ""),
            RawToken::TYPEU64 => (TokenType::TypeU64, ""),
            RawToken::TYPEF32 => (TokenType::TypeF32, ""),
            RawToken::TYPEF64 => (TokenType::TypeF64, ""),
            RawToken::TYPEC32 => (TokenType::TypeC32, ""),
            RawToken::TYPEC64 => (TokenType::TypeC64, ""),
            RawToken::TYPECHAR => (TokenType::TypeChar, ""),
            RawToken::TYPESTRING => (TokenType::TypeString, ""),
            RawToken::TYPEBOOL => (TokenType::TypeBool, ""),
            _ => (TokenType::Unknown, ""),
        }
    }

    pub fn compact_to_string(&self) -> &'static str {
        match self {
            TokenType::Integer => "INT",
            TokenType::Double => "DBL",
            TokenType::Boolean => "BOOL",
            TokenType::Plus => "PLUS_OP",
            TokenType::Minus => "MINUS_OP",
            TokenType::Equal => "EQUAL_OP",
            TokenType::Dot => "DOT_OP",
            TokenType::Star => "STAR_OP",
            TokenType::Divide => "DIVIDE_OP",
            TokenType::Xor => "XOR_OP",
            TokenType::Percent => "PERCENT_OP",
            TokenType::Or => "OR_OP",
            TokenType::And => "AND_OP",
            TokenType::Less => "LESS_OP",
            TokenType::Greater => "GREATER_OP",
            TokenType::PlusPlus => "PLUSPLUS_OP",
            TokenType::MinusMinus => "MINUSMINUS_OP",
            TokenType::PlusEqual => "PLUSEQUAL_OP",
            TokenType::MinusEqual => "MINUSEQUAL_OP",
            TokenType::NotEqual => "NOTEQUAL_OP",
            TokenType::StarEqual => "STAREQUAL_OP",
            TokenType::DivideEqual => "DIVIDEEQUAL_OP",
            TokenType::XorEqual => "XOREQUAL_OP",
            TokenType::PercentEqual => "PERCENTEQUAL_OP",
            TokenType::OrOr => "OROR_OP",
            TokenType::AndAnd => "ANDAND_OP",
            TokenType::EqualEqual => "EQUALEQUAL_OP",
            TokenType::LessEqual => "LESSEQUAL_OP",
            TokenType::GreaterEqual => "GREATEREQUAL_OP",
            TokenType::Identifier => "IDENT",
            TokenType::Char => "CH",
            TokenType::String => "STR",
            TokenType::Eoft => "EOF",
            TokenType::KMain => "K_MAIN",
            TokenType::KVar => "K_VAR",
            TokenType::KIf => "K_IF",
            TokenType::KWhile => "K_WHILE",
            TokenType::KElse => "K_ELSE",
            TokenType::KFor => "K_FOR",
            TokenType::KBreak => "BREAK",
            TokenType::KFun => "K_FUN",
            TokenType::KReturn => "K_RETURN",
            TokenType::KNullptr => "K_NULLPTR",
            TokenType::OpenParenthesis => "OPEN_PAR",
            TokenType::OpenSqParenthesis => "OPEN_SQ_PAR",
            TokenType::OpenCurParenthesis => "OPEN_CUR_PAR",
            TokenType::CloseParenthesis => "CLOSE_PAR",
            TokenType::CloseSqParenthesis => "CLOSE_SQ_PAR",
            TokenType::CloseCurParenthesis => "CLOSE_CUR_PAR",
            TokenType::Not => "NOT_OP",
            TokenType::Comma => "COMMA",
            TokenType::Colon => "COLON",
            TokenType::TypeI8 => "I8",
            TokenType::TypeI16 => "I16",
            TokenType::TypeI32 => "I32",
            TokenType::TypeI64 => "I64",
            TokenType::TypeU8 => "U8",
            TokenType::TypeU16 => "U16",
            TokenType::TypeU32 => "U32",
            TokenType::TypeU64 => "U64",
            TokenType::TypeF32 => "F32",
            TokenType::TypeF64 => "F64",
            TokenType::TypeC32 => "C32",
            TokenType::TypeC64 => "C64",
            TokenType::TypeChar => "CHAR",
            TokenType::TypeString => "STRING",
            TokenType::TypeBool => "BOOL",
            TokenType::Comment => "COMMENT",
            // Add more cases as needed
            TokenType::Unknown => "UNKNOWN",
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.compact_to_string())
        } else {
            // Usa il formato standard.
            write!(f, "{:?}", self)
        }
    }
}
