use super::Block;

#[derive(Debug, Clone)]
pub struct BulletList {
    items: Vec<String>,
}

impl BulletList {
    pub fn new<T: Into<String>>(items: impl IntoIterator<Item = T>) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl Block for BulletList {
    fn render(&self, output: &mut String) {
        use std::fmt::Write;

        for item in &self.items {
            writeln!(output, "- {}", item.trim()).expect("writing to string never fails");
        }

        output.push('\n');
    }
}
