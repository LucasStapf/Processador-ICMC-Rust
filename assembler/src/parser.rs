pub struct Parser<'a> {
    input: &'a String,
    stream: &'a str,
    curr_pos: usize,
    curr_line: usize,
    curr_col: usize,
    lines: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            input,
            stream: input.as_str(),
            curr_pos: 0,
            curr_line: 1,
            curr_col: 0,
            lines: 0,
        }
    }

    /// Pula todos os espaços brancos contidos no `stream`.
    pub fn skip_whitespace(&mut self) {
        loop {
            if self.stream.starts_with(|c| c == '\n') {
                self.new_line();
                self.stream = &self.stream[1..];
            } else if self.stream.starts_with(|c: char| c.is_whitespace()) {
                self.curr_col += 1;
                self.stream = &self.stream[1..];
            } else {
                break;
            }
        }
    }

    pub fn new_line(&mut self) {
        self.curr_line += 1;
        self.curr_col = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace() {
        let s = "\t\t\n CMP R0 R1\n".to_string();
        let mut parser = Parser::new(&s);
        parser.skip_whitespace();
        assert_eq!("CMP R0 R1\n", parser.stream);
    }

    #[test]
    fn test_skip_whitespace_and_count_lines_and_cols() {
        let s = "\t\t\n\n CMP R0 R1\n".to_string();
        let mut parser = Parser::new(&s);
        parser.skip_whitespace();
        assert_eq!("CMP R0 R1\n", parser.stream);
        assert_eq!(3, parser.curr_line);
        assert_eq!(1, parser.curr_col);
    }
}
