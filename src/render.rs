use std::fmt::Write;

use crate::block::Block;

pub(crate) fn render_blocks(output: &mut String, blocks: &[Block], depth: usize) {
    for block in blocks {
        match block {
            Block::Paragraph(text) => {
                writeln!(output, "{}", text.trim()).expect("writing to string never fails");
                output.push('\n');
            }
            Block::BulletList(items) => {
                for item in items {
                    writeln!(output, "- {}", item.trim()).expect("writing to string never fails");
                }
                output.push('\n');
            }
            Block::NumberedList(items) => {
                for item in items {
                    writeln!(output, "+ {}", item.trim()).expect("writing to string never fails");
                }
                output.push('\n');
            }
            Block::Code { language, content } => {
                let lang = language.as_deref().unwrap_or("typst");
                writeln!(output, "```{}", lang).expect("writing to string never fails");
                writeln!(output, "{}", content.trim_end()).expect("writing to string never fails");
                writeln!(output, "```").expect("writing to string never fails");
                output.push('\n');
            }
            Block::Table { headers, rows } => {
                render_table(output, headers, rows);
                output.push('\n');
            }
            Block::Raw(content) => {
                writeln!(output, "{}", content).expect("writing to string never fails");
                output.push('\n');
            }
        }
    }

    if depth > 0 {
        output.push('\n');
    }
}

fn render_table(output: &mut String, headers: &[String], rows: &[Vec<String>]) {
    let column_spec = std::iter::repeat("(flex: 1,)")
        .take(headers.len())
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(output, "#table(columns: ({}))[", column_spec)
        .expect("writing to string never fails");
    output.push_str("  [");
    for (idx, header) in headers.iter().enumerate() {
        if idx > 0 {
            output.push_str("] [");
        }
        output.push_str(header.trim());
    }
    output.push_str("]\n");

    for row in rows {
        output.push_str("  [");
        for (idx, cell) in row.iter().enumerate() {
            if idx > 0 {
                output.push_str("] [");
            }
            output.push_str(cell.trim());
        }
        output.push_str("]\n");
    }

    output.push_str("]\n");
}
