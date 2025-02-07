mod get_state;
mod write_csv;

use crate::get_state::State;
use chrono::Local;
use cross_platform_service::service::start_service;
use service_manager::{ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStopCtx, ServiceUninstallCtx};
use std::env;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

const FREQUENCY: f64 = 1.0; // seconds
const SERVICE_NAME: &str = "WorkLog";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--service") || args.len() <= 1 {
        //env::set_var("WORK_DIR", dir);

        start_service(SERVICE_NAME, |running: Arc<AtomicBool>| {
            {
                let file_result = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("test.txt");
                match file_result {
                    Ok(mut file) => {
                        file.write_all("Service started1".as_bytes()).expect("Error writing to file1");
                        if let Err(e) = writeln!(file, "Service started") {
                            eprintln!("Error writing to file {}: {}", "test.txt", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error opening file {}: {}", "test.txt", e);
                    }
                }
            }

            let mut prev_state = get_state::get_state();
            loop {
                sleep(Duration::from_secs_f64(FREQUENCY));
                if !running.load(Ordering::Relaxed) {
                    return;
                }
                prev_state = iteration(&Vec::new(), &mut prev_state);
            }
       });
    }

    if args.iter().any(|arg| arg == "--cli") {
        let mut prev_state = get_state::get_state();
        loop {
            prev_state = iteration(&args, &mut prev_state);
            sleep(Duration::from_secs_f64(FREQUENCY));
        }
    }

    if args.iter().any(|arg| arg == "--install") {
        println!("Installing service...");
        let manager = <dyn ServiceManager>::native().expect("Failed to detect management platform");
        let label: ServiceLabel = "com.github.daniel-sc.work_log".parse().unwrap();
        let curr_dir = match env::current_dir() {
            Ok(dir) => Some(dir),
            Err(e) => {
                eprintln!("Error getting current directory: {:?}", e);
                None
            }
        };
        println!("using current_dir: {:?}", curr_dir);
        manager
            .install(ServiceInstallCtx {
                label: label.clone(),
                program: env::current_exe().expect("Failed to get current executable path"),
                args: vec![OsString::from("--service")],
                contents: None, // Optional String for system-specific service content.
                username: None, // Optional String for alternative user to run service.
                working_directory: curr_dir.clone(), // not working on win 11
                environment: None, // not working on win!!: Some(Vec::from([(String::from("WORK_DIR"),String::from(curr_dir.expect("No Curr dir!").into_os_string().to_string_lossy()))])), // Optional list of environment variables to supply the service process.
            })
            .expect("Failed to install");
        manager.start(ServiceStartCtx { label }).expect("Failed to start");
    }

    if args.iter().any(|arg| arg == "--uninstall") {
        println!("Uninstalling service...");
        let manager = <dyn ServiceManager>::native().expect("Failed to detect management platform");
        let label: ServiceLabel = "com.github.daniel-sc.work_log".parse().unwrap();
        if let Err(e) = manager.stop(ServiceStopCtx { label: label.clone() }) {
            eprintln!("Failed to stop service: {:?}", e);
        }
        manager
            .uninstall(ServiceUninstallCtx { label })
            .expect("Failed to uninstall");
    }
}

/*
fn run_service(args: &Vec<String>) -> fn(Arc<AtomicBool>) {
     |running: Arc<AtomicBool>| {
        let mut prev_state = get_state::get_state();
        loop {
            if !running.load(Ordering::Relaxed) {
                return;
            }
            prev_state = iteration(&args, &mut prev_state);
            sleep(Duration::from_secs_f64(FREQUENCY));
        }
    }
}*/

fn iteration(args: &Vec<String>, prev_state: &mut State) -> State {
    let current_time = Local::now();
    let state = get_state::get_state();
    if state != *prev_state {
        if args.iter().any(|arg| arg == "--stdout") {
            println!("{}", state.to_csv(&current_time));
        } else {
            /*let work_dir = env::current_exe();
            let file_path = match work_dir {
                Ok(dir) => format!("{}/work_log.csv", dir.to_string_lossy()),
                Err(_) => {
                    String::from("work_log.csv")
                }
            };*/
            write_csv::add_to_csv("C:\\devsbb\\projekte\\work_log\\target\\debug\\work_log.csv", &current_time, &state);
        }
    }
    state
}
