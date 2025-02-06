use chrono::Local;
use std::fs::{create_dir_all, OpenOptions};
use std::path::Path;
use std::io::Write;

/// Appends a log line to the CSV file.
///
/// The log line has the format:
/// `YYYY-MM-DD HH:MM:SS.mmm;window_title_without_semicolon;x,y`
pub(crate) fn add_to_csv(
    csv_filename: &str,
    current_time: &chrono::DateTime<Local>,
    window_title: &str,
    cursor: &str,
) {
    // Format the time to include milliseconds (as in the Python code)
    let ms = current_time.timestamp_subsec_millis();
    let time_str = format!("{}.{:03}", current_time.format("%Y-%m-%d %H:%M:%S"), ms);

    // Remove any semicolons from the window title
    let title_adapted = window_title.replace(";", "");
    let line = format!("{};{};{}", time_str, title_adapted, cursor);

    println!("{}", line);

    // Ensure that the parent directory exists
    if let Some(parent) = Path::new(csv_filename).parent() {
        if !parent.exists() {
            println!("creating {}", parent.display());
            if let Err(e) = create_dir_all(parent) {
                eprintln!("Error creating directory {}: {}", parent.display(), e);
            }
        }
    }

    // Open the file in append mode (or create it if it does not exist)
    let file_result = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_filename);
    match file_result {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", line) {
                eprintln!("Error writing to file {}: {}", csv_filename, e);
            }
        }
        Err(e) => {
            eprintln!("Error opening file {}: {}", csv_filename, e);
        }
    }
}
