pub mod scanner;
pub mod token;

use scanner::Scanner;
use token::{JazonToken, JazonTokenType};

pub fn tokenize(content: String) -> Vec<token::JazonToken> {
    let mut scanner = Scanner::new(&content);
    let mut jazon_token_vec = Vec::new();
    while !scanner.is_done() {
        match scanner.pop() {
            Some(character) => {
                match character {
                    '{' => jazon_token_vec.push(JazonToken::new(JazonTokenType::LBrace, character.to_string())),
                    '}' => jazon_token_vec.push(JazonToken::new(JazonTokenType::RBrace, character.to_string())),
                    _ => jazon_token_vec.push(JazonToken::new(JazonTokenType::Invalid, String::new()))
                }
            }
            None => break
        }
    }
    jazon_token_vec.push(JazonToken::new(JazonTokenType::EOF, String::new()));
    jazon_token_vec
}

#[cfg(test)]
mod tests {
    use self::token::JazonToken;

    use super::*;

    #[test]
    fn test_empty_content() {
        let content = "";
        let expected_tokenization: Vec<token::JazonToken> = vec![JazonToken::new_eof()];

        let tokenized_content = tokenize(content.to_string());
        assert_eq!(tokenized_content, expected_tokenization);
    }

    #[test]
    fn test_brackets_content() {
        let content = "{}";
        let expected_tokenization = vec![
            JazonToken::new(JazonTokenType::LBrace, '{'.to_string()), 
            JazonToken::new(JazonTokenType::RBrace, '}'.to_string()),
            JazonToken::new_eof()
        ];

        let tokenized_content = tokenize(content.to_string());
        assert_eq!(tokenized_content, expected_tokenization);
    }
}