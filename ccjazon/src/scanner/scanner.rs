#[path = "./token/token.rs"]
mod token;

struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }
}
fn tokenize(content: String) -> Vec<token::JazonToken> {
    let scanner = Scanner::new(string);
    println!("{:?}", scanner.characters);
    return true;
}

#[cfg(test)]
mod tests {
    use self::token::JazonToken;

    use super::*;

    #[test]
    fn test_empty_content() {
        let content = "";
        let expected_tokenization: Vec<token::JazonToken> = Vec::new();

        let tokenized_content = tokenize(content.to_string());
        assert_eq!(tokenized_content, expected_tokenization);
    }

    #[test]
    fn test_brackets_content() {
        let content = "{}";
        let expected_tokenization = vec![JazonToken::LBrace, JazonToken::RBrace];

        let tokenized_content = tokenize(content.to_string());
        assert_eq!(tokenized_content, expected_tokenization);
    }
}