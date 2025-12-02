mod block;
mod render;
mod report;
mod section;

#[cfg(feature = "polars")]
pub use block::from_polars_dataframe;
pub use block::{
    Block, BlockNode, Text, TextOptions, bullets, code, numbered, paragraph, raw, table, text,
    text_with_options,
};
pub use report::Report;
pub use section::Section;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    struct DirGuard {
        original: PathBuf,
        temp_dir: PathBuf,
    }

    impl DirGuard {
        fn in_temp(test_name: &str) -> Self {
            let temp_dir = unique_temp_dir(test_name);
            fs::create_dir_all(&temp_dir).expect("should be able to create temp dir");

            let original = env::current_dir().expect("cwd should be available");
            env::set_current_dir(&temp_dir).expect("should be able to set cwd for test");

            Self { original, temp_dir }
        }
    }

    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.original);
            let _ = fs::remove_dir_all(&self.temp_dir);
        }
    }

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "report_creation_test_{}_{}",
            test_name,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be valid")
                .as_millis()
        ))
    }

    #[test]
    fn renders_report_with_outline_and_sections() {
        let _guard = DirGuard::in_temp("renders_report_with_outline_and_sections");

        let report = Report::new("Weekly Status")
            .author("Ada Lovelace")
            .add_front_matter(paragraph("This report summarizes the week."))
            .add_section(
                Section::new("Highlights")
                    .add_block(bullets(["Released v1.2", "Onboarded new teammate"]))
                    .add_subsection(Section::new("Release Details").add_block(paragraph(
                        "The release focused on stability and internal metrics.",
                    ))),
            )
            .add_section(Section::new("Metrics").add_block(table(
                vec!["Key Metric".to_string(), "Value".to_string()],
                vec![vec!["Users".to_string(), "1,024".to_string()]],
            )));

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
    fn sets_page_headers_and_footers() {
        let _guard = DirGuard::in_temp("sets_page_headers_and_footers");

        let report = Report::new("Branded")
            .header("Company Report")
            .footer("Page {{page()}} of {{pages()}}")
            .add_section(
                Section::new("Summary").add_block(paragraph("Quarterly performance overview.")),
            );

        let rendered = report.render();

        assert!(rendered.contains(
            "#set page(header: \"Company Report\", footer: \"Page {{page()}} of {{pages()}}\")"
        ));
    }

    #[test]
    fn supports_code_block_rendering() {
        let _guard = DirGuard::in_temp("supports_code_block_rendering");

        let report = Report::new("Dev Notes")
            .add_section(Section::new("Snippets").add_block(code(Some("rust"), "fn main() {}")));

        let rendered = report.render();

        assert!(rendered.contains("```rust\nfn main() {}\n```"));
    }

    #[test]
    fn validated_render_surfaces_syntax_errors() {
        let invalid_report =
            Report::new("Broken").add_section(Section::new("Faulty").add_block(raw("[#unclosed(")));

        let validation = invalid_report.render_validated();

        assert!(validation.is_err());
        assert!(
            validation
                .unwrap_err()
                .iter()
                .any(|err| err.message.contains("unclosed"))
        );
    }

    #[test]
    fn render_writes_typ_file_using_title() {
        let _guard = DirGuard::in_temp("render_writes_typ_file_using_title");

        let report = Report::new("Build & Ship!")
            .add_section(Section::new("Summary").add_block(paragraph("Ready to go.")));

        let rendered = report.render();
        let typ_path = env::current_dir()
            .expect("should have temp cwd")
            .join("build_ship.typ");
        let saved = fs::read_to_string(&typ_path).expect("render should create typ file");

        assert_eq!(rendered, saved);
    }

    #[test]
    fn render_writes_pdf_when_configured() {
        let _guard = DirGuard::in_temp("render_writes_pdf_when_configured");

        let report = Report::new("PDF please")
            .generate_pdf(true)
            .add_section(Section::new("Summary").add_block(paragraph("PDF output.")));

        report.render();

        let pdf_path = env::current_dir()
            .expect("should have temp cwd")
            .join("pdf_please.pdf");

        let pdf_bytes = fs::read(pdf_path).expect("PDF should be written");
        assert!(!pdf_bytes.is_empty());
    }

    #[test]
    fn paragraphs_accept_text_objects() {
        let shared_text = text("Shared content");

        let rendered = Report::new("Shared Text")
            .add_section(
                Section::new("Body")
                    .add_block(paragraph(shared_text.clone()))
                    .add_block(paragraph(shared_text)),
            )
            .render();

        let expected = "Shared content\n\n";
        assert!(rendered.match_indices(expected).count() >= 2);
    }

    #[test]
    fn renders_formatted_text() {
        let styled = text("Look at me!")
            .fill("red")
            .size("16pt")
            .font("Inter")
            .weight("bold");

        let rendered = Report::new("Style Guide")
            .add_section(Section::new("Body").add_block(paragraph(styled)))
            .render();

        assert!(rendered.contains(
            "#text(\"Look at me!\", fill: red, size: 16pt, font: \"Inter\", weight: bold)",
        ));
    }

    #[test]
    fn accepts_options_struct_for_text() {
        let options = TextOptions::default()
            .lang("en")
            .justification("left")
            .leading("1.4em");

        let rendered = Report::new("Options")
            .add_section(
                Section::new("Body")
                    .add_block(paragraph(text_with_options("Configurable", options))),
            )
            .render();

        assert!(rendered.contains(
            "#text(\"Configurable\", lang: \"en\", justification: left, leading: 1.4em)",
        ));
    }
}
