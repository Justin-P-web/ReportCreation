use super::Block;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    language: Option<String>,
    content: String,
}

impl CodeBlock {
    pub fn new(language: Option<String>, content: String) -> Self {
        Self { language, content }
    }
}

impl Block for CodeBlock {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        let lang = self.language.as_deref().unwrap_or("typst");
        writeln!(output, "```{}", lang).expect("writing to string never fails");
        writeln!(output, "{}", self.content.trim_end()).expect("writing to string never fails");
        writeln!(output, "```").expect("writing to string never fails");
        output.push('\n');
    }
}
