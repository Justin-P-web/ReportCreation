# ReportCreation

A small Rust library for composing Typst reports programmatically. The API uses
simple builders so you can stitch together sections, lists, tables, figures,
links, and code snippets before rendering a ready-to-compile Typst document
string.

## Usage

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
