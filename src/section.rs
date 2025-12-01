use std::fmt::Write;

use crate::{block::BlockNode, render::render_blocks};

/// A section with a heading and a list of content blocks.
#[derive(Debug, Default)]
pub struct Section {
    title: String,
    blocks: Vec<BlockNode>,
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
    pub fn add_block(mut self, block: BlockNode) -> Self {
        self.blocks.push(block);
        self
    }

    /// Add a nested subsection.
    pub fn add_subsection(mut self, section: Section) -> Self {
        self.subsections.push(section);
        self
    }

    pub(crate) fn render(&self, output: &mut String, depth: usize) {
        let heading_level = "=".repeat(depth + 1);
        writeln!(output, "{} {}", heading_level, self.title)
            .expect("writing to string never fails");

        render_blocks(output, &self.blocks, depth);

        for subsection in &self.subsections {
            subsection.render(output, depth + 1);
        }
    }
}
