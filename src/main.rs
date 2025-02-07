mod get_state;
mod write_csv;

use std::env;
use chrono::Local;
use std::thread::sleep;
use std::time::Duration;

const FREQUENCY: f64 = 2.0; // seconds
const ARG_STDOUT: &str = "--stdout";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut prev_state = get_state::get_state();
    loop {
        let current_time = Local::now();
        let state = get_state::get_state();
        if state != prev_state {
            if args.iter().any(|arg| arg == ARG_STDOUT) {
                println!("{}", state.to_csv(&current_time));
            } else {
                let csv_filename = format!("logs/activity_log_{}.csv", current_time.format("%Y_%m_%d"));
                write_csv::add_to_csv(&csv_filename, &current_time, &state);
            }
            prev_state = state;
        }

        sleep(Duration::from_secs_f64(FREQUENCY));
    }
}
