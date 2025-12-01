use super::Block;

#[derive(Debug, Clone)]
pub struct RawBlock {
    content: String,
}

impl RawBlock {
    pub fn new<T: Into<String>>(content: T) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Block for RawBlock {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        writeln!(output, "{}", self.content).expect("writing to string never fails");
        output.push('\n');
    }
}
