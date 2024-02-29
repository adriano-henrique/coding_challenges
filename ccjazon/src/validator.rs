use crate::parser::token::{self, JazonToken};

pub fn validate(tokens: Vec<JazonToken>) -> bool {
    if tokens.len() == 1 {
        return false;
    }

    let mut counter = 0;
    let mut stack: Vec<&JazonToken> = Vec::new();
    while counter < tokens.len() {
        let token = &tokens[counter];
        match token.get_token_type() {
            token::JazonTokenType::LBrace => stack.push(token),
            token::JazonTokenType::RBrace => {
                _ = stack.pop()
            },
            _ => break
        }
        counter += 1;
    }
    stack.len() == 0
}