use active_win_pos_rs::get_active_window;
use chrono::Local;
use device_query::{DeviceQuery, DeviceState};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const SKIP_WRITING_TO_FILE: bool = false;
const FREQUENCY: f64 = 10.0; // seconds

/// Retrieves the current mouse cursor position.
/// Returns a tuple (x, y). If an error occurs, returns None.
fn get_mouse_cursor_position() -> Option<(i32, i32)> {
    let device = DeviceState::new();
    let mouse = device.get_mouse();
    Some((mouse.coords.0, mouse.coords.1))
}

/// Retrieves the title of the active window.
/// If no active window is found or an error occurs, returns an empty String.
fn get_active_window_title() -> String {
    match get_active_window() {
        Ok(window) => format!("{}: {}", window.app_name, window.title),
        Err(e) => {
            eprintln!("Error getting active window: {:?}", e);
            String::new()
        }
    }
}

/// Appends a log line to the CSV file.
///
/// The log line has the format:
/// `YYYY-MM-DD HH:MM:SS.mmm;window_title_without_semicolon;x,y`
fn add_to_csv(
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

    if !SKIP_WRITING_TO_FILE {
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
}

fn main() {
    loop {
        let current_time = Local::now();

        // Get the active window title and mouse cursor position
        let current_window_title = get_active_window_title();
        if let Some((x, y)) = get_mouse_cursor_position() {
            let cursor = format!("{},{}", x, y);
            // Build the filename: logs/activity_log_YYYY_MM_DD.csv
            let csv_filename = format!("logs/activity_log_{}.csv", current_time.format("%Y_%m_%d"));
            add_to_csv(&csv_filename, &current_time, &current_window_title, &cursor);
        } else {
            println!("Error getting mouse cursor position");
        }

        sleep(Duration::from_secs_f64(FREQUENCY));
    }
}
