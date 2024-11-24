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

    pub fn get_brackets_type(value: &str) -> TokenType {
        match value.chars().next() {
            Some('(') => TokenType::OpenParenthesis,
            Some(')') => TokenType::CloseParenthesis,
            Some('[') => TokenType::OpenSqParenthesis,
            Some(']') => TokenType::CloseSqParenthesis,
            Some('{') => TokenType::OpenCurParenthesis,
            Some('}') => TokenType::CloseCurParenthesis,
            _ => TokenType::Unknown,
        }
    }

    pub fn single_char_op(ch: char) -> TokenType {
        match ch {
            '-' => TokenType::Minus,
            '=' => TokenType::Equal,
            '<' => TokenType::Less,
            '>' => TokenType::Greater,
            '!' => TokenType::Not,
            '+' => TokenType::Plus,
            '*' => TokenType::Star,
            '/' => TokenType::Divide,
            '^' => TokenType::Xor,
            '%' => TokenType::Percent,
            '|' => TokenType::Or,
            '&' => TokenType::And,
            _ => TokenType::Unknown,
        }
    }
}
