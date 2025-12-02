mod bullet_list;
mod code;
mod numbered_list;
mod paragraph;
mod raw;
mod table;
mod text;

pub use bullet_list::BulletList;
pub use code::CodeBlock;
pub use numbered_list::NumberedList;
pub use paragraph::Paragraph;
pub use raw::RawBlock;
pub use table::TableBlock;
pub use text::{Text, TextOptions};

pub trait Block: std::fmt::Debug {
    fn render(&self, output: &mut String);
}

pub type BlockNode = Box<dyn Block>;

pub fn text<T: Into<String>>(content: T) -> Text {
    Text::new(content)
}

pub fn text_with_options<T: Into<String>>(content: T, options: TextOptions) -> Text {
    Text::with_options(content, options)
}

pub fn paragraph<T: Into<Text>>(text: T) -> BlockNode {
    Box::new(Paragraph::new(text))
}

pub fn bullets<T: Into<String>>(items: impl IntoIterator<Item = T>) -> BlockNode {
    Box::new(BulletList::new(items))
}

pub fn numbered<T: Into<String>>(items: impl IntoIterator<Item = T>) -> BlockNode {
    Box::new(NumberedList::new(items))
}

pub fn code<T: Into<String>>(language: Option<T>, content: T) -> BlockNode {
    Box::new(CodeBlock::new(language.map(Into::into), content.into()))
}

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

pub fn raw<T: Into<String>>(content: T) -> BlockNode {
    Box::new(RawBlock::new(content))
}

#[cfg(feature = "polars")]
pub fn from_polars_dataframe(
    dataframe: &polars::prelude::DataFrame,
) -> polars::prelude::PolarsResult<BlockNode> {
    TableBlock::from_polars_dataframe(dataframe).map(|table| Box::new(table) as BlockNode)
}
