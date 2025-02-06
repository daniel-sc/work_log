mod get_state;
mod write_csv;

use chrono::Local;
use std::thread::sleep;
use std::time::Duration;

const FREQUENCY: f64 = 10.0; // seconds

fn main() {
    let mut prev_state = get_state::get_state();
    loop {
        let current_time = Local::now();
        let state = get_state::get_state();
        if state != prev_state {
            let cursor = format!("{},{}", state.cursor.0, state.cursor.1);
            let csv_filename = format!("logs/activity_log_{}.csv", current_time.format("%Y_%m_%d"));
            write_csv::add_to_csv(&csv_filename, &current_time, &state.window_title, &cursor);
            prev_state = state;
        }

        sleep(Duration::from_secs_f64(FREQUENCY));
    }
}
