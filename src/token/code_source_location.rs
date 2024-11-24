use std::fmt;

#[derive(Debug, Clone)]
pub struct CodeSourceLocation {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
}

impl CodeSourceLocation {
    pub fn new(file_name: String, line: usize, column: usize) -> Self {
        Self {
            file_name,
            line,
            column,
        }
    }

    pub fn unknown() -> Self {
        Self {
            file_name: "unknown".into(),
            line: 0,
            column: 0,
        }
    }
}

impl fmt::Display for CodeSourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(file: '{}', line: {}, column: {})",
            self.file_name, self.line, self.column
        )
    }
}
