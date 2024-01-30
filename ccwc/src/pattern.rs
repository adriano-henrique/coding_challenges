pub enum CountType {
    LineCount,
    ByteCount,
    WordCount,
}

impl CountType {
    pub fn get_string(&self, content: &String) -> String {
        match self {
            CountType::LineCount => content.lines().count().to_string(),
            CountType::ByteCount => content.len().to_string(),
            CountType::WordCount => content.split_whitespace().count().to_string(),
        }
    }
}
