use super::{Block, Image};

use std::fmt::Write;

#[derive(Debug, Clone)]
pub enum FigureBody {
    Image(Image),
    Table(super::TableBlock),
}

#[derive(Debug, Clone)]
pub enum FigureKind {
    Auto,
    Image,
    Table,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Figure {
    body: FigureBody,
    caption: Option<String>,
    kind: Option<FigureKind>,
}

impl Figure {
    pub fn new(body: impl Into<FigureBody>) -> Self {
        Self {
            body: body.into(),
            caption: None,
            kind: None,
        }
    }

    pub fn caption<T: Into<String>>(mut self, caption: T) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn kind(mut self, kind: FigureKind) -> Self {
        self.kind = Some(kind);
        self
    }
}

impl Block for Figure {
    fn render(&self, output: &mut String) {
        write!(output, "#figure({}", self.body.render_markup())
            .expect("writing to string never fails");

        if let Some(caption) = &self.caption {
            write!(output, ", caption: [{}]", escape_caption(caption))
                .expect("writing to string never fails");
        }

        if let Some(kind) = &self.kind {
            write!(output, ", kind: {}", kind).expect("writing to string never fails");
        }

        writeln!(output, ")").expect("writing to string never fails");
        output.push('\n');
    }
}

impl From<Figure> for super::BlockNode {
    fn from(value: Figure) -> Self {
        Box::new(value)
    }
}

impl FigureBody {
    fn render_markup(&self) -> String {
        match self {
            FigureBody::Image(image) => image.render_markup(false),
            FigureBody::Table(table) => table.render_markup(false),
        }
    }
}

impl From<Image> for FigureBody {
    fn from(value: Image) -> Self {
        FigureBody::Image(value)
    }
}

impl From<super::TableBlock> for FigureBody {
    fn from(value: super::TableBlock) -> Self {
        FigureBody::Table(value)
    }
}

impl std::fmt::Display for FigureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FigureKind::Auto => write!(f, "auto"),
            FigureKind::Image => write!(f, "image"),
            FigureKind::Table => write!(f, "table"),
            FigureKind::Custom(kind) => write!(f, "{}", kind),
        }
    }
}

fn escape_caption(caption: &str) -> String {
    caption
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::TableBlock;

    #[test]
    fn renders_image_figure_with_caption_and_kind() {
        let mut rendered = String::new();
        let figure = Figure::new(Image::new("./diagram.svg").width("50%"))
            .caption("Architecture diagram")
            .kind(FigureKind::Image);

        figure.render(&mut rendered);

        assert_eq!(
            rendered,
            "#figure(image(\"./diagram.svg\", width: 50%), caption: [Architecture diagram], kind: image)\n\n",
        );
    }

    #[test]
    fn renders_table_figure() {
        let mut rendered = String::new();
        let table = TableBlock::new(
            vec!["Column A".to_string(), "Column B".to_string()],
            vec![vec!["1".to_string(), "2".to_string()]],
        );

        Figure::new(table).render(&mut rendered);

        assert!(rendered.starts_with("#figure(table(columns: ((flex: 1,), (flex: 1,)))["));
        assert!(rendered.ends_with(")\n\n"));
    }

    #[test]
    fn escapes_caption_characters() {
        let mut rendered = String::new();
        Figure::new(Image::new("./plot.png"))
            .caption("Bracket [and] slash \\")
            .render(&mut rendered);

        let escaped = escape_caption("Bracket [and] slash \\");
        assert!(rendered.contains(&format!("caption: [{}]", escaped)));
    }
}
