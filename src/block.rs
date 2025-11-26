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

    /// Convenience constructor for raw Typst content.
    pub fn raw<T: Into<String>>(content: T) -> Self {
        Block::Raw(content.into())
    }
}
