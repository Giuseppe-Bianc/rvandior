use super::code_source_location::CodeSourceLocation;
use super::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub source_location: CodeSourceLocation,
}
#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, value: String, source_location: CodeSourceLocation) -> Self {
        Self {
            token_type,
            value,
            source_location,
        }
    }

    pub fn new_with_empty_value(
        token_type: TokenType,
        source_location: CodeSourceLocation,
    ) -> Self {
        Self {
            token_type,
            value: "".to_string(),
            source_location,
        }
    }

    pub fn is_type(&self, token_type: &TokenType) -> bool {
        &self.token_type == token_type
    }

    pub fn is_type_any_of(&self, token_types: &[TokenType]) -> bool {
        token_types.contains(&self.token_type)
    }

    pub fn value_size(&self) -> usize {
        self.value.len()
    }

    pub fn to_compact_string(&self) -> String {
        if self.value.is_empty() {
            format!(
                "(typ: {:#}, sl: {:#})",
                self.token_type, self.source_location
            )
        } else {
            format!(
                "(typ: {:#}, val: '{}', sl: {:#})",
                self.token_type, self.value, self.source_location
            )
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.to_compact_string())
        } else {
            // Usa il formato dettagliato originale.
            if self.value.is_empty() {
                write!(
                    f,
                    "Token(type: {:?}, sourceLocation: {})",
                    self.token_type, self.source_location
                )
            } else {
                write!(
                    f,
                    "Token(type: {:?}, value: '{}', sourceLocation: {})",
                    self.token_type, self.value, self.source_location
                )
            }
        }
    }
}
