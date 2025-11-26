use std::fmt::Write;

pub(crate) fn render_table(output: &mut String, headers: &[String], rows: &[Vec<String>]) {
    let column_spec = std::iter::repeat("(flex: 1,)")
        .take(headers.len())
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(output, "#table(columns: ({}))[", column_spec).expect("writing to string never fails");
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
