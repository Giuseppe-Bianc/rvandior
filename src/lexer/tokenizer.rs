use crate::token::{token_list::TokenList, CodeSourceLocation, RawToken, Token, TokenType};
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

    pub fn tokenize(&mut self) -> TokenList {
        let mut tokenList = TokenList::new(self.file_name.to_string());
        while let Some(raw_token) = self.lexer.next() {
            //let span = lexer.span();
            let value = self.lexer.slice().to_string();
            let (line, column) = self.get_line_and_column();
            let source_location = CodeSourceLocation::new(line, column);
            let (mut token_type, remapped_value) =
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
            tokenList.tokens.push(Token::new(
                token_type,
                remapped_value.to_string(),
                source_location,
            ));
            self.line = line;
            self.column = column + 1;
        }
        tokenList.tokens.push(Token::new(
            TokenType::Eoft,
            "".to_string(),
            CodeSourceLocation::new(self.line, self.column),
        ));
        tokenList
    }
}

//r1<(A, B, C, D, E, F) {(AC)→(ADE), (BD)→(AF), (AB)→(EF), (BF)→(C)}>
