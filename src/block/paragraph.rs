use super::{Block, Text};

#[derive(Debug, Clone)]
pub struct Paragraph {
    content: Text,
}

impl Paragraph {
    pub fn new<T: Into<Text>>(content: T) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Block for Paragraph {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        writeln!(output, "{}", self.content.render()).expect("writing to string never fails");
        output.push('\n');
    }
}
