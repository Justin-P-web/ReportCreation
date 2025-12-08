mod bullet_list;
mod code;
mod figure;
mod image;
mod link;
mod numbered_list;
mod paragraph;
mod raw;
mod table;
mod text;

pub use bullet_list::BulletList;
pub use code::CodeBlock;
pub use figure::{Figure, FigureBody, FigureKind};
pub use image::{Image, ImageOptions};
pub use link::{Link, LinkDestination};
pub use numbered_list::NumberedList;
pub use paragraph::Paragraph;
pub use raw::RawBlock;
pub use table::TableBlock;
pub use text::{Text, TextOptions};

/// Represents a renderable chunk of content that can append Typst markup to a
/// provided output buffer.
///
/// Implementors should focus solely on rendering concerns and avoid mutating
/// external state to keep block composition predictable and testable.
pub trait Block: std::fmt::Debug {
    /// Render the block to the provided string buffer.
    ///
    /// # Arguments
    /// - `output`: Mutable string that receives the rendered Typst markup.
    fn render(&self, output: &mut String);
}

pub type BlockNode = Box<dyn Block>;

/// Create a [`Text`] block with default styling.
///
/// # Arguments
/// - `content`: Raw text to include in the document.
pub fn text<T: Into<String>>(content: T) -> Text {
    Text::new(content)
}

/// Create a [`Text`] block with explicit [`TextOptions`] applied.
///
/// # Arguments
/// - `content`: Raw text to include in the document.
/// - `options`: Style options to apply to the text.
pub fn text_with_options<T: Into<String>>(content: T, options: TextOptions) -> Text {
    Text::with_options(content, options)
}

/// Wrap text content in a paragraph block.
///
/// # Arguments
/// - `text`: Content to place inside the paragraph.
pub fn paragraph<T: Into<Text>>(text: T) -> BlockNode {
    Box::new(Paragraph::new(text))
}

/// Build a bulleted list from the provided items.
///
/// # Arguments
/// - `items`: Iterator of bullet contents.
pub fn bullets<T: Into<String>>(items: impl IntoIterator<Item = T>) -> BlockNode {
    Box::new(BulletList::new(items))
}

/// Build a numbered list from the provided items.
///
/// # Arguments
/// - `items`: Iterator of list entries to number.
pub fn numbered<T: Into<String>>(items: impl IntoIterator<Item = T>) -> BlockNode {
    Box::new(NumberedList::new(items))
}

/// Create a code block with an optional language tag.
///
/// # Arguments
/// - `language`: Optional language identifier for syntax highlighting.
/// - `content`: Source code to render inside the block.
pub fn code<T: Into<String>>(language: Option<T>, content: T) -> BlockNode {
    Box::new(CodeBlock::new(language.map(Into::into), content.into()))
}

/// Create an image block with the provided image options.
///
/// # Arguments
/// - `image`: Image descriptor including path and sizing information.
pub fn image<I: Into<Image>>(image: I) -> BlockNode {
    Box::new(image.into())
}

/// Construct a figure wrapper around content such as an image or code block.
///
/// # Arguments
/// - `body`: Renderable content to include inside the figure.
pub fn figure(body: impl Into<FigureBody>) -> Figure {
    Figure::new(body)
}

/// Create a hyperlink pointing to an external URL.
///
/// # Arguments
/// - `url`: Destination URL.
/// - `content`: Visible link text.
pub fn link_to_url<C: Into<Text>, U: Into<String>>(url: U, content: C) -> BlockNode {
    Box::new(Link::to_url(url, content))
}

/// Create an internal document link pointing to a Typst location.
///
/// # Arguments
/// - `location`: Location anchor within the Typst document.
/// - `content`: Visible link text.
pub fn link_to_location<C: Into<Text>, L: Into<String>>(location: L, content: C) -> BlockNode {
    Box::new(Link::to_location(location, content))
}

/// Create a table block from headers and row data.
///
/// # Arguments
/// - `headers`: Column headers displayed at the top of the table.
/// - `rows`: Row iterators containing cell values.
pub fn table<H, R, C>(
    headers: impl IntoIterator<Item = H>,
    rows: impl IntoIterator<Item = R>,
) -> BlockNode
where
    H: Into<String>,
    R: IntoIterator<Item = C>,
    C: Into<String>,
{
    Box::new(TableBlock::new(headers, rows))
}

/// Insert raw Typst content without escaping or additional formatting.
///
/// # Arguments
/// - `content`: Raw Typst markup to insert directly into the document.
pub fn raw<T: Into<String>>(content: T) -> BlockNode {
    Box::new(RawBlock::new(content))
}

#[cfg(feature = "polars")]
/// Render a Polars data frame as a table block.
///
/// # Arguments
/// - `dataframe`: Source data to render.
///
/// # Errors
/// Propagates Polars errors that occur while reading the frame.
pub fn from_polars_dataframe(
    dataframe: &polars::prelude::DataFrame,
) -> polars::prelude::PolarsResult<BlockNode> {
    TableBlock::from_polars_dataframe(dataframe).map(|table| Box::new(table) as BlockNode)
}
