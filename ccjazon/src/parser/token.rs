#[derive(PartialEq, Debug, Clone)]
pub struct JazonToken {
    token_type: JazonTokenType,
    literal: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum JazonTokenType {
    LBrace,
    RBrace,
    EOF,
    Invalid,
}

impl JazonToken {
    pub fn new(token_type: JazonTokenType, literal: String) -> JazonToken {
        JazonToken { token_type, literal }
    }

    pub fn new_eof() -> JazonToken {
        JazonToken { token_type: JazonTokenType::EOF, literal: String::new() }
    }

    pub fn new_invalid() -> JazonToken {
        JazonToken { token_type: JazonTokenType::Invalid, literal: String::new() }
    }

    pub fn get_token_type(&self) -> JazonTokenType {
        self.token_type
    }
}
