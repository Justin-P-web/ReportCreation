use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use ReportCreation as reportcreation;

fn write_typst_fixture(dir: &std::path::Path) -> std::path::PathBuf {
    let typst_path = dir.join("sample.typ");
    fs::write(
        &typst_path,
        "#set document(title: \"CLI Test\")\nThis is a test report.",
    )
    .expect("fixture should be written");

    typst_path
}

#[test]
fn writes_pdf_next_to_input_by_default() {
    let temp_dir = tempdir().expect("tempdir should be created");
    let input_path = write_typst_fixture(temp_dir.path());
    let expected_output = input_path.with_extension("pdf");

    Command::new(assert_cmd::cargo::cargo_bin!("report_creation"))
        .current_dir(temp_dir.path())
        .arg(&input_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            expected_output.display().to_string(),
        ));

    let pdf_bytes = fs::read(expected_output).expect("pdf should be written by CLI");
    assert!(!pdf_bytes.is_empty());
}

#[test]
fn honors_custom_output_path() {
    let temp_dir = tempdir().expect("tempdir should be created");
    let input_path = write_typst_fixture(temp_dir.path());
    let custom_output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&custom_output_dir).expect("custom output dir should be creatable");
    let custom_output = custom_output_dir.join("custom.pdf");

    Command::new(assert_cmd::cargo::cargo_bin!("report_creation"))
        .current_dir(temp_dir.path())
        .arg(&input_path)
        .arg("--output")
        .arg(&custom_output)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            custom_output.display().to_string(),
        ));

    assert!(
        fs::metadata(&custom_output).is_ok(),
        "custom pdf should exist"
    );
}

#[test]
fn accepts_relative_input_path() {
    let temp_dir = tempdir().expect("tempdir should be created");
    let input_path = write_typst_fixture(temp_dir.path());
    let relative_input = input_path
        .strip_prefix(temp_dir.path())
        .expect("input should be within temp dir");
    let relative_output = relative_input.with_extension("pdf");
    let expected_output = temp_dir.path().join(&relative_output);

    Command::new(assert_cmd::cargo::cargo_bin!("report_creation"))
        .current_dir(temp_dir.path())
        .arg(relative_input)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            relative_output.display().to_string(),
        ));

    assert!(
        fs::metadata(&expected_output).is_ok(),
        "pdf should be written"
    );
}

#[test]
fn resolves_imports_relative_to_input_file() {
    let temp_dir = tempdir().expect("tempdir should be created");

    let template_path = temp_dir.path().join("template.typ");
    fs::write(&template_path, "#let greeting = \"Hello from template\"\n").expect("template should be written");

    let nested_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&nested_dir).expect("nested dir should be created");

    let input_path = nested_dir.join("import_test.typ");
    fs::write(
        &input_path,
        "#import \"../template.typ\": greeting\n#greeting\n",
    )
    .expect("main typst file should be written");

    let pdf_bytes = reportcreation::compile_pdf(
        &fs::read_to_string(&input_path).expect("source should be readable"),
        &input_path,
    );

    assert!(!pdf_bytes.is_empty(), "pdf should be generated when import resolves");
}
