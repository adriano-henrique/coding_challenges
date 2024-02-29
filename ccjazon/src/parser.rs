pub mod token;
pub mod scanner;

use token::{JazonToken, JazonTokenType};

pub fn tokenize(content: String) -> Vec<token::JazonToken> {
    let mut jazon_token_vec = Vec::new();
    for c in content.chars() {
        match c {
            '{' => jazon_token_vec.push(JazonToken::new(JazonTokenType::LBrace, c.to_string())),
            '}' => jazon_token_vec.push(JazonToken::new(JazonTokenType::RBrace, c.to_string())),
            _ => jazon_token_vec.push(JazonToken::new_invalid())
        }
    }
    jazon_token_vec.push(JazonToken::new_eof());
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