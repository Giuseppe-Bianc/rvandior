use lexer::Tokenizer;

mod lexer;
mod token;

fn main() {
    let token = token::Token::new(
        token::TokenType::KMain,
        "main".to_string(),
        token::CodeSourceLocation::new("hine.vn".to_string(), 3, 3),
    );
    println!("{}", token);
    if token.token_type.is_keyword() {
        println!("Token is a keyword");
    } else {
        println!("Token is not a keyword");
    }
    println!("Token value size: {}", token.value_size());
    let _unkowm_code_source_location = token::CodeSourceLocation::unknown();
    let mut lexer = Tokenizer::new("hine.vn", "main");
    let tokens = lexer.tokenize();
    if tokens.is_empty() {
        println!("No token found");
    } else {
        for token in tokens {
            println!("{}", token);
        }
    }
}
