#[cfg(feature = "mock-open")]
use crate::mockopen::open;
#[cfg(not(feature = "mock-open"))]
use open;
use std::collections::HashMap;
use std::io::Error;
use std::slice::Iter;
use std::thread::sleep;
use std::{fs, time};

use build_html::Html;
use chrono::{NaiveDate, TimeDelta};
use tabled::grid::records::vec_records::{Cell, CellInfo};
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
    println!("Opening in browser...");
    sleep(time::Duration::from_millis(2000));

    Ok(())
}

fn to_html_table(table: tabled::Table) -> build_html::Table {
    let mut html_rows: Vec<Vec<String>> = vec![];
    let mut table_rows = table.get_records().iter_rows().peekable();
    let mut i = 0;
    while let Some(table_row) = table_rows.next() {
        let mut html_row: Vec<String> = vec![];
        let row_iter = table_row.iter();

        let is_button_row = i >= 5;
        if table_rows.peek().is_none() {
            html_row = to_html_row(row_iter, time_to_decimal_string_flex);
        } else if is_button_row {
            html_row = to_html_row(row_iter, time_to_decimal_string_normal);
        } else {
            for cell in row_iter {
                html_row.push(cell.text().to_string())
            }
        }
        html_rows.push(html_row);
        i += 1;
    }
    build_html::Table::from(html_rows)
}

fn to_html_row<F>(mut row_iter: Iter<'_, CellInfo<String>>, time_to_string: F) -> Vec<String>
where
    F: Fn(String) -> String,
{
    let mut html_row: Vec<String> = vec![];
    let first_cell = row_iter.next().expect("all rows have at least one cell");
    let cells_except_first: Vec<String> = row_iter
        .clone()
        .map(|cell| cell.text().to_string())
        .map(time_to_string)
        .collect();
    let cells_except_first_with_tab_separators = cells_except_first.join("\t");
    let first_cell_text = format!(
        "<button onclick=\"copyToClipboard('{}')\">{}</button>",
        cells_except_first_with_tab_separators,
        first_cell.text()
    );
    html_row.push(first_cell_text);
    for string in cells_except_first {
        html_row.push(string)
    }
    html_row
}

fn time_to_decimal_string_normal(time_str: String) -> String {
    time_to_decimal_string(time_str, false)
}

fn time_to_decimal_string_flex(time_str: String) -> String {
    time_to_decimal_string(time_str, true)
}

fn time_to_decimal_string(time_str: String, flex: bool) -> String {
    // An empty string returns an empty string
    if time_str.is_empty() {
        return String::new();
    }
    let is_negative = time_str.starts_with('-');
    let parts: Vec<&str> = time_str.trim_start_matches('-').split(':').collect();

    // Handle format validation
    if parts.len() != 2 {
        return format!("Error: Invalid format '{}'. Use HH:MM", time_str);
    }

    // Parse hours with error handling
    let hours = match parts[0].parse::<u32>() {
        Ok(h) if h < 24 => h,
        Ok(_) => return format!("Error: Hours in '{}' must be < 24", time_str),
        Err(_) => return format!("Error: Invalid hours '{}'", parts[0]),
    };

    // Parse minutes with error handling
    let minutes = match parts[1].parse::<u32>() {
        Ok(m) if m < 60 => m,
        Ok(_) => return format!("Error: Minutes in '{}' must be < 60", time_str),
        Err(_) => return format!("Error: Invalid minutes '{}'", parts[1]),
    };

    // Calculate and format the result
    let mut decimal = hours as f64 + (minutes as f64 / 60.0);
    if is_negative {
        decimal = -decimal;
    }
    if flex {
        if decimal > 0.0 {
            return String::new();
        } else if decimal < 0.0 {
            decimal = -decimal;
        }
    }
    format!("{:.2}", decimal).replace('.', ",")
}
