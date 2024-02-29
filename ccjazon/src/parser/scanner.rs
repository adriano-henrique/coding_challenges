use crate::parser::JazonToken;

#[derive(Clone)]
pub struct Scanner {
    cursor: usize,
    tokens: Vec<JazonToken>,
}

impl Scanner {
    pub fn new(token_vec: &Vec<JazonToken>) -> Self {
        Self {
            cursor: 0,
            tokens: token_vec.to_vec(),
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&JazonToken> {
        self.tokens.get(self.cursor)
    }

    pub fn is_done(&self) -> bool {
        self.cursor == self.tokens.len()
    }

    pub fn pop(&mut self) -> Option<&JazonToken> {
        match self.tokens.get(self.cursor) {
            Some(token) => {
                self.cursor += 1;

                Some(token)
            }
            None => None,
        }
    }
}
