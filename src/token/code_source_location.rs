use std::fmt;

#[derive(Debug, Clone)]
pub struct CodeSourceLocation {
    pub line: usize,
    pub column: usize,
}

impl CodeSourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn to_compact_string(&self) -> String {
        format!("(ln: {}, cln: {})", self.line, self.column)
    }
}

impl fmt::Display for CodeSourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Usa il formato compatto con "{:#}".
            write!(f, "{}", self.to_compact_string())
        } else {
            // Usa il formato standard.
            write!(f, "(line: {}, column: {})", self.line, self.column)
        }
    }
}
