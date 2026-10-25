mod get_state;
#[cfg(any(target_os = "linux", test))]
mod gnome_extension;
mod write_csv;

use crate::get_state::State;
use auto_launch::{AutoLaunchBuilder, MacOSLaunchMode};
use chrono::Local;
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_FREQUENCY: f64 = 2.0; // seconds
const ARG_STDOUT: &str = "--stdout";
const AUTOSTART_APP_NAME: &'static str = "Work Log";

fn parse_frequency(value: &str) -> Result<Duration, &'static str> {
    value
        .parse::<f64>()
        .ok()
        .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok())
        .filter(|duration| !duration.is_zero())
        .ok_or(
            "--frequency must be a positive number of seconds representable as a nonzero duration",
        )
}

fn autostart_args(args: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    args.iter()
        .filter(|arg| *arg != "--autostart-install")
        .map(|arg| {
            if let Some(file) = arg.strip_prefix("--file=") {
                let path = std::path::absolute(file)?;
                let path = path
                    .to_str()
                    .ok_or("autostart file path is not valid UTF-8")?;
                Ok(format!("--file={path}"))
            } else {
                Ok(arg.clone())
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--help") {
        println!("Usage: work_log [OPTION]...");
        println!("Log the active window and cursor position to a CSV file.");
        println!();
        println!("Options:");
        println!("  --frequency=SECONDS  set the logging interval (positive; default: 2.0)");
        println!("  --file=FILE          set the file to write to (default: ~/work_log.csv)");
        println!("  --stdout             print to stdout instead of a file");
        println!("  --autostart-install  install the program to run at startup.");
        println!("    (options are preserved, do not move the executable afterwards)");
        println!("  --autostart-remove   remove the program from startup");
        println!("  --help               display this help and exit");
        return Ok(());
    }

    let interval = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--frequency="))
        .map(parse_frequency)
        .transpose()?
        .unwrap_or(Duration::from_secs_f64(DEFAULT_FREQUENCY));

    if args.iter().any(|arg| arg == "--autostart-install") {
        let args = autostart_args(&args)?;
        AutoLaunchBuilder::new()
            .set_app_name(AUTOSTART_APP_NAME)
            .set_app_path(
                env::current_exe()
                    .expect("could not locate current executable path")
                    .to_str()
                    .expect("could not convert path to string"),
            )
            .set_args(args.as_slice())
            .set_macos_launch_mode(MacOSLaunchMode::LaunchAgent)
            .build()?
            .enable()?;
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--autostart-remove") {
        AutoLaunchBuilder::new()
            .set_app_name(AUTOSTART_APP_NAME)
            .set_app_path(
                env::current_exe()
                    .expect("could not locate current executable path")
                    .to_str()
                    .expect("could not convert path to string"),
            )
            .set_args(&[] as &[&str])
            .set_macos_launch_mode(MacOSLaunchMode::LaunchAgent)
            .build()?
            .disable()?;
        return Ok(());
    }

    let user_home = env::var("HOME").unwrap_or(env::var("USERPROFILE").unwrap_or(".".to_string()));
    let default_file = Path::new(&user_home).join(Path::new("work_log.csv"));
    let file = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--file="))
        .map(Path::new)
        .unwrap_or(&default_file);
    let use_stdout = args.iter().any(|arg| arg == ARG_STDOUT);
    if use_stdout {
        eprintln!("Logging to stdout");
    } else {
        eprintln!("Logging to {}", file.display());
    }

    // On GNOME Wayland, ensure the shell extension for active window detection is set up
    #[cfg(target_os = "linux")]
    gnome_extension::ensure_gnome_wayland_extension();

    // Start background mouse listener for cursor position tracking
    get_state::start_mouse_listener();

    let mut prev_state: Option<State> = None;
    loop {
        let current_time = Local::now();
        let state = get_state::get_state();
        if prev_state.as_ref() != Some(&state) {
            if use_stdout {
                println!("{}", state.to_csv(&current_time));
            } else {
                write_csv::add_to_csv(file, &current_time, &state);
            }
            prev_state = Some(state);
        }

        sleep(interval);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_accepts_positive_representable_intervals() {
        assert_eq!(parse_frequency("2"), Ok(Duration::from_secs(2)));
        assert_eq!(parse_frequency("0.25"), Ok(Duration::from_millis(250)));
        assert_eq!(parse_frequency("1e-9"), Ok(Duration::from_nanos(1)));
    }

    #[test]
    fn frequency_rejects_invalid_or_zero_intervals() {
        for value in ["", "abc", "1=2", "-1", "0", "NaN", "inf", "1e100", "1e-100"] {
            assert!(parse_frequency(value).is_err(), "accepted {value:?}");
        }
    }

    #[test]
    fn autostart_preserves_options_and_resolves_new_output_paths() {
        let dir = tempfile::tempdir().unwrap();
        let absolute = dir.path().join("missing").join("report=2026.csv");
        let relative = Path::new("missing").join("report=2026.csv");
        for path in [relative.as_path(), absolute.as_path()] {
            let args = vec![
                "--autostart-install".to_owned(),
                format!("--file={}", path.display()),
                "--frequency=0.25".to_owned(),
            ];
            let resolved = if path.is_absolute() {
                path.to_owned()
            } else {
                env::current_dir().unwrap().join(path)
            };
            assert_eq!(
                autostart_args(&args).unwrap(),
                [
                    format!("--file={}", resolved.display()),
                    "--frequency=0.25".to_owned(),
                ]
            );
        }
        assert!(!absolute.exists());
    }
}
