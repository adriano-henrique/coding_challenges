pub struct JazonToken {
    token_type: JazonTokenType,
    literal: String,
}

enum JazonTokenType {
    LBrace,
    RBrace,
    EOF,
}
