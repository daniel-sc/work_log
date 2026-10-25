use chrono::Local;
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

impl State {
    pub fn to_csv(&self, time: &chrono::DateTime<Local>) -> String {
        format!(
            "{},{},{},{},{}",
            time.format("%+"),
            csv_field(&self.app_name),
            csv_field(&self.window_title),
            self.cursor.0,
            self.cursor.1
        )
    }
}

fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Shared cursor position, updated by the mouse listener thread.
static CURSOR_POS: RwLock<(i32, i32)> = RwLock::new((0, 0));

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
            app_name: window.info.name,
            window_title: window.title,
            cursor,
        },
        Err(e) => {
            eprintln!("Error getting active window: {}", e);
            State {
                app_name: String::from(""),
                window_title: String::from(""),
                cursor,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_csv_fields_without_losing_text() {
        let time = chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05Z")
            .unwrap()
            .with_timezone(&Local);
        for (text, encoded) in [
            ("", ""),
            ("Editor – 日本語", "Editor – 日本語"),
            ("App, Inc.", "\"App, Inc.\""),
            ("a\"b", "\"a\"\"b\""),
            ("first\nsecond", "\"first\nsecond\""),
            ("first\rsecond", "\"first\rsecond\""),
            ("a,\"b\"\r\nc", "\"a,\"\"b\"\"\r\nc\""),
        ] {
            let state = State {
                app_name: text.to_owned(),
                window_title: text.to_owned(),
                cursor: (-10, 20),
            };
            assert_eq!(
                state.to_csv(&time),
                format!("{},{encoded},{encoded},-10,20", time.format("%+")),
                "input: {text:?}",
            );
        }
    }
}
