use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use time::{OffsetDateTime, UtcOffset};

use crate::{block::BlockNode, render::render_blocks, section::Section};
use comemo::Prehashed;
use typst::{
    Library, World, compile,
    diag::{FileError, FileResult},
    eval::Tracer,
    foundations::{Bytes, Smart},
    syntax::{FileId, SyntaxError, parse},
    text::{Font, FontBook},
};
use typst_assets::fonts;
use typst_pdf::pdf;

/// Represents a report composed of structured sections and blocks that can be
/// rendered to Typst markup.
#[derive(Debug, Default)]
pub struct Report {
    title: String,
    author: Option<String>,
    header: Option<String>,
    footer: Option<String>,
    include_outline: bool,
    include_figure_table: bool,
    generate_pdf: bool,
    sections: Vec<Section>,
    front_matter: Vec<BlockNode>,
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
            include_figure_table: false,
            generate_pdf: false,
            sections: Vec::new(),
            front_matter: Vec::new(),
        }
    }

    /// Configure whether a PDF should be generated alongside the Typst output.
    pub fn generate_pdf(mut self, generate_pdf: bool) -> Self {
        self.generate_pdf = generate_pdf;
        self
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

    /// Configure whether a table of figures should be included after the
    /// outline. Defaults to `false`.
    pub fn with_figure_table(mut self, include_figure_table: bool) -> Self {
        self.include_figure_table = include_figure_table;
        self
    }

    /// Add content that should appear before any section headings.
    pub fn add_front_matter(mut self, block: BlockNode) -> Self {
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
        let rendered = self.render_validated().unwrap_or_else(|errors| {
            let summary = errors
                .iter()
                .map(|err| err.message.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            panic!("generated Typst markup contains syntax errors: {}", summary)
        });

        let file_name = typst_file_name(&self.title);
        let file_path = std::env::current_dir()
            .unwrap_or_else(|err| panic!("failed to resolve current directory: {}", err))
            .join(&file_name);

        fs::write(&file_path, &rendered).unwrap_or_else(|err| {
            panic!(
                "failed to write Typst output to {}: {}",
                file_path.display(),
                err
            )
        });

        if self.generate_pdf {
            let pdf_bytes = compile_pdf(&rendered, &file_path);
            let pdf_file = pdf_file_name(&self.title);

            fs::write(&pdf_file, &pdf_bytes).unwrap_or_else(|err| {
                panic!("failed to write PDF output to {}: {}", pdf_file, err)
            });
        }

        rendered
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

        output.push_str(figure_table_function());

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

        if self.include_figure_table {
            writeln!(output, "= Table of Figures").expect("writing to string never fails");
            output.push_str("#figure_table()\n\n");
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

fn figure_table_function() -> &'static str {
    r#"#let figure_table() = {
  let entries = query(selector: figure);

  if entries.len() == 0 {
    []
  } else {
    table(
      columns: (auto, 1fr),
      align: (right, left),
      ..entries.map(entry => [
        link(entry.location(), [entry.counter.display()])
        entry.caption
      ])
    )
  }
}

"#
}

fn typst_file_name(title: &str) -> String {
    format!("{}.typ", normalized_stem(title))
}

fn pdf_file_name(title: &str) -> String {
    format!("{}.pdf", normalized_stem(title))
}

fn normalized_stem(title: &str) -> String {
    let normalized = title
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else if ch.is_whitespace() || ch == '-' {
                '_'
            } else {
                '_'
            }
        })
        .collect::<String>();

    let compacted = normalized
        .split('_')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    let stem = if compacted.is_empty() {
        "report".to_string()
    } else {
        compacted
    };

    stem
}

struct InMemoryWorld {
    source: typst::syntax::Source,
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    fonts: Vec<Font>,
    root: PathBuf,
}

impl InMemoryWorld {
    fn new(source: String, main_path: PathBuf) -> Self {
        let root = main_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        let main_id = FileId::new(
            None,
            typst::syntax::VirtualPath::within_root(&main_path, &root)
                .unwrap_or_else(|| typst::syntax::VirtualPath::new(&main_path)),
        );

        let source = typst::syntax::Source::new(main_id, source);

        let fonts: Vec<Font> = fonts()
            .flat_map(|data| Font::iter(Bytes::from(data.to_vec())))
            .collect();
        let book = FontBook::from_fonts(&fonts);

        Self {
            source,
            library: Prehashed::new(Library::default()),
            book: Prehashed::new(book),
            fonts,
            root,
        }
    }
}

impl World for InMemoryWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> typst::syntax::Source {
        self.source.clone()
    }

    fn source(&self, id: FileId) -> FileResult<typst::syntax::Source> {
        if id == self.source.id() {
            return Ok(self.source.clone());
        }

        let path = id
            .vpath()
            .resolve(&self.root)
            .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))?;

        let text = fs::read_to_string(&path)
            .map_err(|_| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))?;

        Ok(typst::syntax::Source::new(id, text))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let path = id
            .vpath()
            .resolve(&self.root)
            .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))?;

        fs::read(path)
            .map(Bytes::from)
            .map_err(|_| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        let now = match offset {
            Some(hours) => {
                let seconds = hours.checked_mul(3600)?;
                let utc_offset = UtcOffset::from_whole_seconds(seconds.try_into().ok()?).ok()?;
                OffsetDateTime::now_utc().to_offset(utc_offset)
            }
            None => OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc()),
        };

        typst::foundations::Datetime::from_ymd_hms(
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        )
    }
}

fn compile_pdf(source: &str, main_path: &Path) -> Vec<u8> {
    let world = InMemoryWorld::new(source.to_string(), main_path.to_path_buf());
    let mut tracer = Tracer::new();
    let document = compile(&world, &mut tracer)
        .unwrap_or_else(|err| panic!("failed to compile Typst document to PDF: {err:?}"));

    pdf(&document, Smart::Auto, None)
}
