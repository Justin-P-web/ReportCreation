use std::fmt::Write;

use crate::block::Block;

use super::table::render_table;

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
