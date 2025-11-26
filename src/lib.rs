mod block;
mod render;
mod report;
mod section;

pub use block::Block;
pub use report::Report;
pub use section::Section;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_report_with_outline_and_sections() {
        let report = Report::new("Weekly Status")
            .author("Ada Lovelace")
            .add_front_matter(Block::paragraph("This report summarizes the week."))
            .add_section(
                Section::new("Highlights")
                    .add_block(Block::bullets(["Released v1.2", "Onboarded new teammate"]))
                    .add_subsection(Section::new("Release Details").add_block(Block::paragraph(
                        "The release focused on stability and internal metrics.",
                    ))),
            )
            .add_section(Section::new("Metrics").add_block(Block::Table {
                headers: vec!["Key Metric".into(), "Value".into()],
                rows: vec![vec!["Users".into(), "1,024".into()]],
            }));

        let rendered = report.render();

        assert!(
            rendered.contains("#set document(title: \"Weekly Status\", author: \"Ada Lovelace\"")
        );
        assert!(rendered.contains("#outline()"));
        assert!(rendered.contains("= Weekly Status"));
        assert!(rendered.contains("== Highlights"));
        assert!(rendered.contains("=== Release Details"));
        assert!(rendered.contains("- Released v1.2"));
        assert!(rendered.contains("#table"));
    }

    #[test]
    fn supports_code_block_rendering() {
        let report = Report::new("Dev Notes").add_section(
            Section::new("Snippets").add_block(Block::code(Some("rust"), "fn main() {}")),
        );

        let rendered = report.render();

        assert!(rendered.contains("```rust\nfn main() {}\n```"));
    }
}
