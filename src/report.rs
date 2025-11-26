use std::fmt::Write;

use crate::{block::Block, render::render_blocks, section::Section};

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
