use std::fmt::Write;

pub(crate) fn table_markup(headers: &[String], rows: &[Vec<String>], include_hash: bool) -> String {
    let mut output = String::new();
    let column_spec = std::iter::repeat("(flex: 1,)")
        .take(headers.len())
        .collect::<Vec<_>>()
        .join(", ");
    let prefix = if include_hash { "#table" } else { "table" };
    writeln!(output, "{}(columns: ({}))[", prefix, column_spec)
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
    output
}
