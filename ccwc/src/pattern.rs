pub struct Pattern {
    value: Option<String>,
}

impl Pattern {
    pub fn new(content: Option<String>) -> Pattern {
        Pattern { value: content }
    }

    pub fn get_pattern_type(&self) -> PatternType {
        match &self.value {
            Some(pattern) => match pattern.as_str() {
                "-c" => return PatternType::ByteCount,
                "-l" => return PatternType::LineCount,
                "-w" => return PatternType::WordCount,
                _ => return PatternType::Invalid,
            },
            None => return PatternType::NotProvided,
        }
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
            PatternType::ByteCount => write!(f, "byte_count (-c)"),
            PatternType::LineCount => write!(f, "line_count (-l)"),
            PatternType::WordCount => write!(f, "word_count (-w)"),
            PatternType::NotProvided => write!(f, "specific pattern ommitted"),
            _ => write!(f, "invalid type"),
        }
    }
}

impl PatternType {
    pub fn to_string(&self, content: &String) -> String {
        match self {
            PatternType::LineCount => content.lines().count().to_string(),
            PatternType::ByteCount => content.len().to_string(),
            PatternType::WordCount => content.split_whitespace().count().to_string(),
            PatternType::NotProvided => build_not_provided_string(&content),
            PatternType::Invalid => String::new(),
        }
    }
}

fn build_not_provided_string(content: &String) -> String {
    let mut ans = String::new();

    ans = ans
        + &" "
        + &PatternType::ByteCount.to_string(content)
        + &" "
        + &PatternType::LineCount.to_string(content)
        + &" "
        + &PatternType::WordCount.to_string(content);

    return ans;
}
