use crate::token::{CodeSourceLocation, RawToken, Token, TokenType};
use logos::{Lexer, Logos};

#[allow(dead_code)]
pub struct Tokenizer<'a> {
    input: &'a str,
    file_name: &'a str,
    line: usize,
    column: usize,
    lexer: Lexer<'a, RawToken>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(file_name: &'a str, input: &'a str) -> Self {
        Self {
            file_name: file_name,
            input,
            line: 0,
            column: 0,
            lexer: RawToken::lexer(input),
        }
    }

    fn get_line_and_column(&mut self) -> (usize, usize) {
        let byte_index = self.lexer.span().start;
        let lines: Vec<&str> = self.input[..byte_index].split('\n').collect();
        let line_number = lines.len();
        let column_number = lines.last().unwrap_or(&"").chars().count();
        (line_number, column_number)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(raw_token) = self.lexer.next() {
            //let span = lexer.span();
            let value = self.lexer.slice().to_string();
            let (line, column) = self.get_line_and_column();
            let source_location = CodeSourceLocation::new(self.file_name.to_string(), line, column);
            let (mut token_type, _remapped_value) =
                TokenType::map_raw_token_type(raw_token.clone().unwrap(), &value);
            match raw_token.clone().unwrap() {
                RawToken::Number => {
                    if value.contains('.') {
                        token_type = TokenType::Double;
                    } else {
                        token_type = TokenType::Integer;
                    }
                }
                _ => {}
            };
            tokens.push(Token::new(token_type, value, source_location));
            self.line = line;
            self.column = column;
        }
        tokens.push(Token::new(
            TokenType::Eoft,
            "".to_string(),
            CodeSourceLocation::new(self.file_name.to_string(), self.line, self.column),
        ));
        tokens
    }
}

//r1<(A, B, C, D, E, F) {(AC)→(ADE), (BD)→(AF), (AB)→(EF), (BF)→(C)}>
