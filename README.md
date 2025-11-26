# ReportCreation

A small Rust library for composing Typst reports programmatically. The API uses
simple builders so you can stitch together sections, lists, tables, and code
snippets before rendering a ready-to-compile Typst document string.

## Usage

Add the crate to your project (path dependency shown for local development):

```toml
[dependencies]
report_creation = { path = "." }
```

Build up a report and render it to Typst markup:

```rust
use report_creation::{Block, Report, Section};

let document = Report::new("Weekly Status")
    .author("Ada Lovelace")
    .add_front_matter(Block::paragraph("Summary of the week's work."))
    .add_section(
        Section::new("Highlights").add_block(Block::bullets([
            "Released v1.2",
            "Onboarded a new teammate",
        ])),
    )
    .add_section(
        Section::new("Metrics").add_block(Block::table(
            ["Key Metric", "Value"],
            [
                ["Users", "1,024"],
                ["Error Budget", "99.98%"],
            ],
        )),
    )
    .render();

println!("{}", document);
```

The rendered Typst document includes a `#set document(..)` declaration, an
outline by default, nested section headings, and the blocks you composed. You
can disable the outline with `Report::with_outline(false)` and add nested
subsections with `Section::add_subsection`.

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
use report_creation::{Block, Report};

let df = df!(
    "Feature" => ["Adoption", "Churn"],
    "Value" => [0.81, 0.07],
)?;

let document = Report::new("Metrics")
    .add_block(Block::from_polars_dataframe(&df)?)
    .render();
```

Each column name becomes a table header, and values are stringified row-by-row
in the rendered Typst output.
