use std::fmt::Write;

/// Represents a report composed of structured sections and blocks that can be
/// rendered to Typst markup.
#[derive(Debug, Default)]
pub struct Report {
    title: String,
    author: Option<String>,
    include_outline: bool,
    sections: Vec<Section>,
    front_matter: Vec<Block>,
}

impl Report {
    /// Create a new report with a title.
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            author: None,
            include_outline: true,
            sections: Vec::new(),
            front_matter: Vec::new(),
        }
    }

    /// Set the author for the report.
    pub fn author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Configure whether an outline should be included at the start of the
    /// rendered Typst document. Defaults to `true`.
    pub fn with_outline(mut self, include_outline: bool) -> Self {
        self.include_outline = include_outline;
        self
    }

    /// Add content that should appear before any section headings.
    pub fn add_front_matter(mut self, block: Block) -> Self {
        self.front_matter.push(block);
        self
    }

    /// Add a section to the report.
    pub fn add_section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    /// Render the report to a Typst document string.
    pub fn render(&self) -> String {
        let mut output = String::new();

        writeln!(
            output,
            "#set document(title: \"{}\"{}\n)",
            self.title,
            render_author(self.author.as_deref())
        )
        .expect("writing to string never fails");

        writeln!(output, "= {}", self.title).expect("writing to string never fails");

        if self.include_outline {
            output.push_str("#outline()\n\n");
        }

        render_blocks(&mut output, &self.front_matter, 0);

        for section in &self.sections {
            section.render(&mut output, 1);
        }

        output
    }
}

fn render_author(author: Option<&str>) -> String {
    match author {
        Some(name) => format!(", author: \"{}\"", name),
        None => String::new(),
    }
}

/// A section with a heading and a list of content blocks.
#[derive(Debug, Default)]
pub struct Section {
    title: String,
    blocks: Vec<Block>,
    subsections: Vec<Section>,
}

impl Section {
    /// Create a section with the provided title.
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            blocks: Vec::new(),
            subsections: Vec::new(),
        }
    }

    /// Add a block of content to the section.
    pub fn add_block(mut self, block: Block) -> Self {
        self.blocks.push(block);
        self
    }

    /// Add a nested subsection.
    pub fn add_subsection(mut self, section: Section) -> Self {
        self.subsections.push(section);
        self
    }

    fn render(&self, output: &mut String, depth: usize) {
        let heading_level = "=".repeat(depth + 1);
        writeln!(output, "{} {}", heading_level, self.title)
            .expect("writing to string never fails");

        render_blocks(output, &self.blocks, depth);

        for subsection in &self.subsections {
            subsection.render(output, depth + 1);
        }
    }
}

/// Discrete content units that can be composed inside sections.
#[derive(Debug, Clone)]
pub enum Block {
    Paragraph(String),
    BulletList(Vec<String>),
    NumberedList(Vec<String>),
    Code {
        language: Option<String>,
        content: String,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    /// Raw Typst content that is passed through unchanged.
    Raw(String),
}

impl Block {
    /// Convenience constructor for a paragraph block.
    pub fn paragraph<T: Into<String>>(text: T) -> Self {
        Block::Paragraph(text.into())
    }

    /// Convenience constructor for a bullet list block.
    pub fn bullets<T: Into<String>>(items: impl IntoIterator<Item = T>) -> Self {
        Block::BulletList(items.into_iter().map(Into::into).collect())
    }

    /// Convenience constructor for a numbered list block.
    pub fn numbered<T: Into<String>>(items: impl IntoIterator<Item = T>) -> Self {
        Block::NumberedList(items.into_iter().map(Into::into).collect())
    }

    /// Convenience constructor for a code block.
    pub fn code<T: Into<String>>(language: Option<T>, content: T) -> Self {
        Block::Code {
            language: language.map(Into::into),
            content: content.into(),
        }
    }

    /// Convenience constructor for a table block.
    pub fn table<H, R, C>(
        headers: impl IntoIterator<Item = H>,
        rows: impl IntoIterator<Item = R>,
    ) -> Self
    where
        H: Into<String>,
        R: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Block::Table {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
        }
    }
}

fn render_blocks(output: &mut String, blocks: &[Block], depth: usize) {
    for block in blocks {
        match block {
            Block::Paragraph(text) => {
                writeln!(output, "{}", text.trim()).expect("writing to string never fails");
                output.push('\n');
            }
            Block::BulletList(items) => {
                for item in items {
                    writeln!(output, "- {}", item.trim()).expect("writing to string never fails");
                }
                output.push('\n');
            }
            Block::NumberedList(items) => {
                for item in items {
                    writeln!(output, "+ {}", item.trim()).expect("writing to string never fails");
                }
                output.push('\n');
            }
            Block::Code { language, content } => {
                let lang = language.as_deref().unwrap_or("typst");
                writeln!(output, "```{}", lang).expect("writing to string never fails");
                writeln!(output, "{}", content.trim_end()).expect("writing to string never fails");
                writeln!(output, "```").expect("writing to string never fails");
                output.push('\n');
            }
            Block::Table { headers, rows } => {
                render_table(output, headers, rows);
                output.push('\n');
            }
            Block::Raw(content) => {
                writeln!(output, "{}", content).expect("writing to string never fails");
                output.push('\n');
            }
        }
    }

    if depth > 0 {
        output.push('\n');
    }
}

fn render_table(output: &mut String, headers: &[String], rows: &[Vec<String>]) {
    let column_spec: String = std::iter::repeat("(flex: 1,) ")
        .take(headers.len())
        .collect::<String>();
    writeln!(output, "#table(columns: ({}),", column_spec.trim_end())
        .expect("writing to string never fails");
    output.push_str("  [");
    for (idx, header) in headers.iter().enumerate() {
        if idx > 0 {
            output.push_str("] [");
        }
        output.push_str(header.trim());
    }
    output.push_str("],\n");

    for row in rows {
        output.push_str("  [");
        for (idx, cell) in row.iter().enumerate() {
            if idx > 0 {
                output.push_str("] [");
            }
            output.push_str(cell.trim());
        }
        output.push_str("],\n");
    }

    output.push_str(")\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_report_with_outline_and_sections() {
        let report = Report::new("Weekly Status")
            .author("Ada Lovelace")
            .add_front_matter(Block::paragraph("This report summarizes the week."))
            .add_section(
                Section::new("Highlights")
                    .add_block(Block::bullets(["Released v1.2", "Onboarded new teammate"]))
                    .add_subsection(Section::new("Release Details").add_block(Block::paragraph(
                        "The release focused on stability and internal metrics.",
                    ))),
            )
            .add_section(Section::new("Metrics").add_block(Block::Table {
                headers: vec!["Key Metric".into(), "Value".into()],
                rows: vec![vec!["Users".into(), "1,024".into()]],
            }));

        let rendered = report.render();

        assert!(
            rendered.contains("#set document(title: \"Weekly Status\", author: \"Ada Lovelace\"")
        );
        assert!(rendered.contains("#outline()"));
        assert!(rendered.contains("= Weekly Status"));
        assert!(rendered.contains("== Highlights"));
        assert!(rendered.contains("=== Release Details"));
        assert!(rendered.contains("- Released v1.2"));
        assert!(rendered.contains("#table"));
    }

    #[test]
    fn supports_code_block_rendering() {
        let report = Report::new("Dev Notes").add_section(
            Section::new("Snippets").add_block(Block::code(Some("rust"), "fn main() {}")),
        );

        let rendered = report.render();

        assert!(rendered.contains("```rust\nfn main() {}\n```"));
    }
}
