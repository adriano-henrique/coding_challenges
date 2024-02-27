use crate::parser::token::JazonToken;

pub fn validate(tokens: Vec<JazonToken>) -> bool {
    if tokens.len() == 1 {
        return false;
    }
    true
}