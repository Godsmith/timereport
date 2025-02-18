#[cfg(feature = "mock-open")]
use crate::mockopen::open;
#[cfg(not(feature = "mock-open"))]
use open;
use std::collections::HashMap;
use std::io::Error;
use std::thread::sleep;
use std::{fs, time};

use build_html::Html;
use chrono::{NaiveDate, TimeDelta};
use tabled::grid::records::vec_records::Cell;
use tabled::grid::records::Records;
use tempfile::tempdir;

use crate::day::Day;
use crate::naive_date::one_date_per_week;
use crate::table::create_table;

const JAVASCRIPT: &str = "
<script>
const copyToClipboard = str => {
  const el = document.createElement('textarea');
  el.value = str;
  el.setAttribute('readonly', '');
  el.style.position = 'absolute';
  el.style.left = '-9999px';
  document.body.appendChild(el);
  el.select();
  document.execCommand('copy');
  document.body.removeChild(el);
};
</script>
";

pub fn create_html_table(
    first_date: NaiveDate,
    last_date: NaiveDate,
    day_from_date: &HashMap<NaiveDate, Day>,
    show_weekend: bool,
    project_names: &Vec<String>,
    working_time_per_day: &TimeDelta,
) -> Result<(), Error> {
    let html: String = one_date_per_week(first_date, last_date)
        .iter()
        .map(|date| {
            to_html_table(create_table(
                *date,
                &day_from_date,
                show_weekend,
                &project_names,
                working_time_per_day,
            ))
            .to_html_string()
        })
        .collect();
    let tmp_dir = tempdir()?;
    let path = tmp_dir.path().join("tmp.html");
    let javascript_plus_html = JAVASCRIPT.to_string() + &html;
    fs::write(&path, javascript_plus_html)?;
    open::that(path)?;
    // Sleep here so that the browser has time to load the file before it
    // is deleted. Kind of hacky.
    sleep(time::Duration::from_millis(200));

    Ok(())
}

fn to_html_table(table: tabled::Table) -> build_html::Table {
    let mut rows: Vec<Vec<String>> = vec![];
    for (i, table_row) in table.get_records().iter_rows().enumerate() {
        let mut html_row: Vec<String> = vec![];
        let mut row_iterator = table_row.iter();

        let is_button_row = i >= 5;
        if is_button_row {
            let first_cell = row_iterator
                .next()
                .expect("all rows have at least one cell");
            let row_iterator2 = row_iterator.clone();
            let row_text_with_tabs: String = row_iterator2
                .map(|cell| cell.text().to_string())
                .collect::<Vec<String>>()
                .join("\t");
            let first_cell_text = format!(
                "<button onclick=\"copyToClipboard('{}')\">{}</button>",
                row_text_with_tabs,
                first_cell.text()
            );
            html_row.push(first_cell_text);
        };

        for cell in row_iterator {
            html_row.push(cell.text().to_string())
        }
        rows.push(html_row)
    }
    build_html::Table::from(rows)
}
