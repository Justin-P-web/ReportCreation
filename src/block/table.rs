use super::Block;

use crate::render::table::render_table;

#[derive(Debug, Clone)]
pub struct TableBlock {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl TableBlock {
    pub fn new<H, R, C>(
        headers: impl IntoIterator<Item = H>,
        rows: impl IntoIterator<Item = R>,
    ) -> Self
    where
        H: Into<String>,
        R: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
        }
    }

    #[cfg(feature = "polars")]
    pub fn from_polars_dataframe(
        dataframe: &polars::prelude::DataFrame,
    ) -> polars::prelude::PolarsResult<Self> {
        let headers = dataframe
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect();

        let mut rows = Vec::with_capacity(dataframe.height());
        for row_idx in 0..dataframe.height() {
            let mut row = Vec::with_capacity(dataframe.width());
            for column in dataframe.get_columns() {
                let value = column.get(row_idx)?;
                row.push(value.to_string());
            }
            rows.push(row);
        }

        Ok(Self { headers, rows })
    }
}

impl Block for TableBlock {
    fn render(&self, output: &mut String) {
        render_table(output, &self.headers, &self.rows);
        output.push('\n');
    }
}
