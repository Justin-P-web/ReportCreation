# ReportCreation

A small Rust library (and CLI) for composing Typst reports programmatically. The
API uses simple builders so you can stitch together sections, lists, tables,
figures, links, and code snippets before rendering a ready-to-compile Typst
document string.

## Features

- Builder-based API for constructing rich Typst documents
- Optional `polars` feature to turn `DataFrame`s into tables
- Built-in Typst compilation helper and CLI to produce PDFs without installing
  the Typst toolchain separately
- Optional outline, table of contents, and table of figures generation

## Library usage

Add the crate to your project (path dependency shown for local development):

```toml
[dependencies]
report_creation = { path = "." }
```

Build up a report and render it to Typst markup:

```rust
use report_creation::{
    bullets, code, figure, image, link_to_url, numbered, paragraph, table, text, Image,
    Report, Section,
};

let document = Report::new("Weekly Status")
    .author("Ada Lovelace")
    .header("Acme Corp | Weekly Status")
    .footer("Page {{page()}} of {{pages()}}")
    .with_contents_table(true)
    .with_figure_table(true)
    .add_front_matter(paragraph("Summary of the week's work."))
    .add_section(
        Section::new("Highlights")
            .add_block(bullets([
                "Released v1.2",
                "Onboarded a new teammate",
            ]))
            .add_block(numbered(["Follow up with marketing", "Retro action items"]))
            .add_block(link_to_url("https://example.com", text("See more"))),
    )
    .add_section(
        Section::new("Metrics")
            .add_block(code(Some("rust"), "fn main() {}"))
            .add_block(table(
                ["Key Metric", "Value"],
                [
                    ["Users", "1,024"],
                    ["Error Budget", "99.98%"],
                ],
            ))
            .add_block(
                figure(Image::new("./chart.svg").width("75%"))
                    .caption("Quarterly performance")
                    .into(),
            ),
    )
    .render();

println!("{}", document);
```

`Report::render` returns the Typst markup string and writes a `.typ` file using a
normalized version of the title (for example, `Weekly Status` becomes
`weekly_status.typ`). Call `Report::generate_pdf(true)` to additionally write a
compiled PDF alongside the Typst output.

### Rendering and PDF output

Use `Report::render` when you want the Typst source and file written to disk.
If you already have Typst source, you can call `compile_pdf(source, path)` to
produce PDF bytes using the embedded Typst engine and fonts, without needing a
separate Typst installation.

You can disable the outline with `Report::with_outline(false)`, add reusable page
chrome with `Report::header` and `Report::footer`, add a table of contents with
`Report::with_contents_table(true)`, include a table of figures via
`Report::with_figure_table(true)`, and add nested subsections with
`Section::add_subsection`. Headers and footers accept a `PageSection` built from
blocks (strings are automatically wrapped in a paragraph), so you can stitch
together richer page chrome.

## Blocks

The crate exports helper constructors so you can build content quickly:

- `paragraph` for rich text blocks built from `Text`
- `bullets` and `numbered` for lists
- `table` for tabular data (and `from_polars_dataframe` when the `polars`
  feature is enabled)
- `code` for fenced code blocks (defaults to the `typst` language when omitted)
- `image` and `figure` for visual content
- `link_to_url` and `link_to_location` for hyperlinks
- `raw` for injecting Typst directly

Each helper returns a `BlockNode` so you can chain `Section::add_block` calls.

## Turning Polars DataFrames into Typst tables

Enable the optional `polars` feature to convert a `polars::prelude::DataFrame`
into a rendered table block:

```toml
[dependencies]
report_creation = { path = ".", features = ["polars"] }
polars = { version = "0.44", default-features = false, features = ["fmt"] }
```

Then build the table straight from your DataFrame:

```rust
use polars::prelude::*;
use report_creation::{table, Report};

let df = df!(
    "Feature" => ["Adoption", "Churn"],
    "Value" => [0.81, 0.07],
)?;

let document = Report::new("Metrics")
    .add_block(table(df.get_column_names(), df.iter_rows()))
    .render();
```

Each column name becomes a table header, and values are stringified row-by-row
in the rendered Typst output.

## Command-line PDF compiler

The repository also ships a small CLI that compiles an existing Typst document
into a PDF using the same embedded Typst engine as the library. If you've
already built the binary (for example, via `cargo build --release`) and have it
available as `./target/release/report_creation` (or `report_creation.exe` on
Windows), invoke it directly from your shell:

```bash
./target/release/report_creation path/to/input.typ --output path/to/output.pdf
```

When `--output` is omitted, the CLI writes a PDF next to the input file with the
`.pdf` extension. This can be handy for testing the generated Typst output
without installing the Typst CLI separately.

## Simple Typst quickstart

If you're new to Typst and want to try the generated documents locally, you can
use the prebuilt CLI packaged with this repository to compile your first file
without installing Typst separately:

1. Review the [official Typst documentation](https://typst.app/docs/) for
   syntax and layout basics.
2. Create a new file (for example, `hello.typ`) with minimal content:
   ```typst
   #set page(width: 8.5in, height: 11in)
   = Hello Typst

   This document was generated with Typst.
   ```
3. Compile the Typst file into a PDF using the built-in compiler (assuming the
   executable is available as `./target/release/report_creation` or
   `report_creation.exe` in your shell):
   ```bash
   ./target/release/report_creation hello.typ --output hello.pdf
   ```
4. Open `hello.pdf` with your preferred PDF viewer to verify the output.

This workflow mirrors how the library and CLI in this repository emit Typst
source and compile it to PDF using the embedded Typst engine.
