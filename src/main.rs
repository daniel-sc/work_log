mod get_state;
mod write_csv;

use crate::get_state::State;
use auto_launch::AutoLaunchBuilder;
use chrono::Local;
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_FREQUENCY: f64 = 2.0; // seconds
const ARG_STDOUT: &str = "--stdout";
const AUTOSTART_APP_NAME: &'static str = "Work Log";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--help") {
        println!("Usage: work_log [OPTION]...");
        println!("Log the active window and cursor position to a CSV file.");
        println!();
        println!("Options:");
        println!("  --frequency=SECONDS  set the frequency of logging (default: 2.0)");
        println!("  --file=FILE          set the file to write to (default: ~/work_log.csv)");
        println!("  --stdout             print to stdout instead of a file");
        println!("  --autostart-install  install the program to run at startup.");
        println!("    (options are preserved, do not move the executable afterwards)");
        println!("  --autostart-remove   remove the program from startup");
        println!("  --help               display this help and exit");
        return Ok(());
    }

    if args.iter().any(|arg| arg == "--autostart-install") {
        let vec = args
            .iter()
            .filter(|arg| *arg != "--autostart-install")
            // adapt relative paths to absolute paths:
            .map(|arg| {
                if arg.starts_with("--file=") {
                    let abs = env::current_dir()
                        .expect("could not locate current directory")
                        .join(Path::new(&arg[7..]))
                        .canonicalize()
                        .expect("could not canonicalize path")
                        .to_str()
                        .expect("could not convert path to string")
                        .to_string();
                    format!("--file={}", abs)
                } else {
                    arg.clone()
                }
            })
            .collect::<Vec<String>>();
        AutoLaunchBuilder::new()
            .set_app_name(AUTOSTART_APP_NAME)
            .set_app_path(env::current_exe().expect("could not locate current executable path").to_str().expect("could not convert path to string"))
            .set_args(vec.as_slice())
            .set_use_launch_agent(false)
            .build()?
            .enable()?;
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--autostart-remove") {
        AutoLaunchBuilder::new()
            .set_app_name(AUTOSTART_APP_NAME)
            .set_app_path(env::current_exe().expect("could not locate current executable path").to_str().expect("could not convert path to string"))
            .set_args(&[] as &[&str])
            .set_use_launch_agent(false)
            .build()?
            .disable()?;
        return Ok(());
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
