use std::{default, fmt::write};

struct Pattern {
    value: Option<String>,
}

impl Pattern {
    pub fn get_pattern_type(&self) -> PatternType {
        match self.value {
            Some(pattern) => match pattern.as_str() {
                "-c" => return PatternType::ByteCount,
                "-l" => return PatternType::LineCount,
                "-w" => return PatternType::WordCount,
                _ => return PatternType::Invalid,
            },
            None => return PatternType::NotProvided,
        }
    }

    pub fn get_pattern_string(&self) -> String {
        return self.get_pattern_type().to_string(&self.value);
    }
}

pub enum PatternType {
    LineCount,
    ByteCount,
    WordCount,
    NotProvided,
    Invalid,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PatternType::ByteCount => write!(f, "write_count"),
            PatternType::LineCount => write!(f, "line_count"),
            PatternType::WordCount => write!(f, "word_count"),
            _ => write!(f, "invalid type"),
        }
    }
}

impl PatternType {
    // Add a general type error
    pub fn get_error_message(&self) -> String {
        return format!("The content value should not be null for command: {}", self);
    }

    pub fn to_string(&self, content: &Option<String>) -> String {
        match self {
            PatternType::LineCount => content
                .expect(&PatternType::LineCount.get_error_message())
                .lines()
                .count()
                .to_string(),
            PatternType::ByteCount => content
                .expect(&PatternType::LineCount.get_error_message())
                .len()
                .to_string(),
            PatternType::WordCount => content
                .expect(&PatternType::WordCount.get_error_message())
                .split_whitespace()
                .count()
                .to_string(),
            PatternType::NotProvided => String::new(),
            PatternType::Invalid => String::new(),
        }
    }
}
