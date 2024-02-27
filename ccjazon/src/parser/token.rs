#[derive(PartialEq, Debug)]
pub struct JazonToken {
    token_type: JazonTokenType,
    literal: String,
}

#[derive(PartialEq, Debug)]
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
}
