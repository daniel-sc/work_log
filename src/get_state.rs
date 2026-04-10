use chrono::Local;
use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};
use std::sync::RwLock;
use std::thread;
use x_win::get_active_window;

// TODO react to keyboard input (e.g. typing)

#[derive(Debug, PartialEq)]
pub struct State {
    pub app_name: String,
    pub window_title: String,
    pub cursor: (i32, i32),
}

const SEPARATOR: &str = ",";

impl State {
    pub fn to_csv(&self, time: &chrono::DateTime<Local>) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            time.format("%+"),
            SEPARATOR,
            self.app_name.replace(SEPARATOR, ""),
            SEPARATOR,
            self.window_title.replace(SEPARATOR, ""),
            SEPARATOR,
            self.cursor.0,
            SEPARATOR,
            self.cursor.1
        )
    }
}

/// Shared cursor position, updated by the mouse listener thread.
static CURSOR_POS: Lazy<RwLock<(i32, i32)>> = Lazy::new(|| RwLock::new((0, 0)));

/// Starts a background thread that listens for mouse move events and updates CURSOR_POS.
/// This function should be called once at program startup.
/// On Linux with Wayland, the user must be in the 'input' group for this to work.
pub fn start_mouse_listener() {
    thread::spawn(|| {
        if let Err(e) = listen(|event: Event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                if let Ok(mut pos) = CURSOR_POS.write() {
                    *pos = (x as i32, y as i32);
                }
            }
        }) {
            eprintln!("Warning: Failed to start mouse listener: {:?}", e);
            eprintln!("Cursor position tracking may not work. On Linux, ensure you are in the 'input' group.");
        }
    });
}

/// Check if the x-win GNOME Shell extension directory exists on disk.
/// We avoid `x_win::is_installed_extension()` because it has a bug in v5.6
/// that calls `disable_extension()` instead of checking installation status.
fn is_extension_installed_on_disk() -> bool {
    if let Ok(home) = std::env::var("HOME") {
        std::path::Path::new(&home)
            .join(".local/share/gnome-shell/extensions/x-win@miniben90.org")
            .is_dir()
    } else {
        false
    }
}

/// On GNOME Wayland, active window detection requires a GNOME Shell extension.
/// This function auto-installs the extension if needed, then probes whether
/// `get_active_window()` actually works. We avoid `x_win::is_installed_extension()`
/// and `x_win::is_enabled_extension()` because both are broken in x-win v5.6:
/// - `is_installed_extension()` accidentally calls `disable_extension()`
/// - `is_enabled_extension()` fails to deserialize the D-Bus `a{sv}` response
pub fn ensure_gnome_wayland_extension() {
    let is_wayland = std::env::var("XDG_SESSION_TYPE")
        .map(|s| s == "wayland")
        .unwrap_or(false);
    let is_gnome = std::env::var("XDG_CURRENT_DESKTOP")
        .map(|s| s.to_lowercase().contains("gnome"))
        .unwrap_or(false);

    if !is_wayland || !is_gnome {
        return;
    }

    if !is_extension_installed_on_disk() {
        eprintln!("GNOME Wayland detected: installing x-win shell extension...");
        match x_win::install_extension() {
            Ok(_) => {
                eprintln!("Extension installed to ~/.local/share/gnome-shell/extensions/x-win@miniben90.org");
                eprintln!("");
                eprintln!("To complete setup:");
                eprintln!("  1. Log out and log back in to load the extension");
                eprintln!("  2. Enable the extension:");
                eprintln!("     gnome-extensions enable x-win@miniben90.org");
                eprintln!("  3. Restart this program");
                eprintln!("");
                eprintln!("Active window detection will not work until these steps are completed.");
            }
            Err(e) => {
                eprintln!("Warning: Failed to install GNOME shell extension: {:?}", e);
                eprintln!("Active window detection will not work on GNOME Wayland.");
                eprintln!(
                    "You can try manually installing from: https://github.com/miniben-90/x-win"
                );
            }
        }
        return;
    }

    // Probe: try an actual get_active_window() call to see if the extension works.
    // Retry several times because on login (autostart) the GNOME Shell extension
    // may still be loading when we start.
    let mut working = false;
    for attempt in 0..10 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        if get_active_window().is_ok() {
            working = true;
            break;
        }
    }
    if !working {
        eprintln!("GNOME Wayland: x-win shell extension is installed but active window");
        eprintln!("detection is not working. Make sure the extension is enabled:");
        eprintln!("  gnome-extensions enable x-win@miniben90.org");
        eprintln!("You may need to log out and back in after enabling.");
        eprintln!("");
        eprintln!("The program will continue but window info may be empty.");
    }
}

pub(crate) fn get_state() -> State {
    let cursor = CURSOR_POS.read().map(|pos| *pos).unwrap_or((0, 0));
    match get_active_window() {
        Ok(window) => State {
            app_name: window.info.name,
            window_title: window.title,
            cursor,
        },
        Err(e) => {
            eprintln!("Error getting active window: {:?}", e);
            State {
                app_name: String::from(""),
                window_title: String::from(""),
                cursor,
            }
        }
    }
}
