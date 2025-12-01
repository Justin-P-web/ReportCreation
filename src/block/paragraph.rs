use super::Block;

#[derive(Debug, Clone)]
pub struct Paragraph {
    content: String,
}

impl Paragraph {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Block for Paragraph {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        writeln!(output, "{}", self.content.trim()).expect("writing to string never fails");
        output.push('\n');
    }
}
