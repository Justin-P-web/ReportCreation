use std::fmt::Write;

use crate::{block::Block, render::render_blocks, section::Section};
use typst::syntax::{SyntaxError, parse};

/// Represents a report composed of structured sections and blocks that can be
/// rendered to Typst markup.
#[derive(Debug, Default)]
pub struct Report {
    title: String,
    author: Option<String>,
    header: Option<String>,
    footer: Option<String>,
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
            header: None,
            footer: None,
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

    /// Configure a page header for the report.
    pub fn header<T: Into<String>>(mut self, header: T) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Configure a page footer for the report.
    pub fn footer<T: Into<String>>(mut self, footer: T) -> Self {
        self.footer = Some(footer.into());
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
        self.render_validated().unwrap_or_else(|errors| {
            let summary = errors
                .iter()
                .map(|err| err.message.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            panic!("generated Typst markup contains syntax errors: {}", summary)
        })
    }

    /// Render the report to Typst markup, returning syntax errors if the
    /// generated output is invalid Typst.
    pub fn render_validated(&self) -> Result<String, Vec<SyntaxError>> {
        let mut output = String::new();

        writeln!(
            output,
            "#set document(title: \"{}\"{})",
            self.title,
            render_author(self.author.as_deref())
        )
        .expect("writing to string never fails");

        if self.header.is_some() || self.footer.is_some() {
            writeln!(
                output,
                "#set page({})",
                render_page(self.header.as_deref(), self.footer.as_deref())
            )
            .expect("writing to string never fails");
        }

        writeln!(output, "= {}", self.title).expect("writing to string never fails");

        if self.include_outline {
            output.push_str("#outline()\n\n");
        }

        render_blocks(&mut output, &self.front_matter, 0);

        for section in &self.sections {
            section.render(&mut output, 1);
        }

        let parsed = parse(&output);
        let errors = parsed.errors();

        if errors.is_empty() {
            Ok(output)
        } else {
            Err(errors)
        }
    }
}

fn render_author(author: Option<&str>) -> String {
    match author {
        Some(name) => format!(", author: \"{}\"", name),
        None => String::new(),
    }
}

fn render_page(header: Option<&str>, footer: Option<&str>) -> String {
    let mut parts = Vec::new();

    if let Some(header_content) = header {
        parts.push(format!(
            "header: \"{}\"",
            escape_typst_string(header_content)
        ));
    }

    if let Some(footer_content) = footer {
        parts.push(format!(
            "footer: \"{}\"",
            escape_typst_string(footer_content)
        ));
    }

    parts.join(", ")
}

fn escape_typst_string(raw: &str) -> String {
    raw.replace('\\', "\\\\").replace('"', "\\\"")
}
