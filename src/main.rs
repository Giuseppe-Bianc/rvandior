use std::path::Path;

use lexer::Tokenizer;

mod lexer;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("C:/dev/visualStudio/transpiler/Vandior/input.vn");
    if let Some(path_str) = file_path.to_str() {
        let contents = match std::fs::read_to_string(file_path) {
            Ok(it) => it,
            Err(err) => return Err(Box::new(err)),
        };
        let mut lexer = Tokenizer::new(path_str, &contents);
        let tokens = lexer.tokenize();
        if tokens.tokens.is_empty() {
            eprintln!("No token found");
        } else {
            println!("file_name: {}", tokens.file_name);
            println!("Tokens:");
            for token in &tokens.tokens {
                println!("{:#}", token);
            }
            if tokens.tokens.len() > 100 {
                println!("file_name: {}", tokens.file_name);
            }
        }
    } else {
        eprintln!("Path is not valid UTF-8");
    }
    Ok(())
}
