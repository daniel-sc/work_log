mod get_state;
mod write_csv;

use crate::get_state::State;
use chrono::Local;
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_FREQUENCY: f64 = 2.0; // seconds
const ARG_STDOUT: &str = "--stdout";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--help") {
        println!("Usage: work_log [OPTION]...");
        println!("Log the active window and cursor position to a CSV file.");
        println!();
        println!("Options:");
        println!("  --frequency=SECONDS  set the frequency of logging (default: 2.0)");
        println!("  --file=FILE          set the file to write to (default: ~/work_log.csv)");
        println!("  --stdout             print to stdout instead of a file");
        return;
    }
    let freq = args
        .iter()
        .find(|arg| arg.starts_with("--frequency="))
        .map(|arg| arg.split('=').last().unwrap())
        .map(|arg| arg.parse::<f64>().unwrap_or(DEFAULT_FREQUENCY))
        .unwrap_or(DEFAULT_FREQUENCY);
    let user_home = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap_or(".".to_string()));
    let default_file = Path::new(&user_home).join(Path::new("work_log.csv"));
    let file = args
        .iter()
        .find(|arg| arg.starts_with("--file="))
        .map(|arg| {
            Path::new(
                arg.split('=')
                    .last()
                    .expect(&format!("Invalid argument: {}", arg)),
            )
        })
        .unwrap_or(&default_file);
    println!("Logging to {}", file.display());

    let mut prev_state: State = get_state::get_state();
    loop {
        let current_time = Local::now();
        let state = get_state::get_state();
        if state != prev_state {
            if args.iter().any(|arg| arg == ARG_STDOUT) {
                println!("{}", state.to_csv(&current_time));
            } else {
                write_csv::add_to_csv(file, &current_time, &state);
            }
            prev_state = state;
        }

        sleep(Duration::from_secs_f64(freq));
    }
}
