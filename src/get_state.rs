use active_win_pos_rs::get_active_window;
use chrono::Local;
use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};
use std::sync::RwLock;
use std::thread;

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

pub(crate) fn get_state() -> State {
    let cursor = CURSOR_POS.read().map(|pos| *pos).unwrap_or((0, 0));
    match get_active_window() {
        Ok(window) => State {
            app_name: window.app_name,
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
