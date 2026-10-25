use chrono::Local;
use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use x_win::get_active_window;

// TODO react to keyboard input (e.g. typing)

#[derive(Debug, PartialEq)]
pub struct State {
    pub app_name: String,
    pub window_title: String,
}

impl State {
    pub fn to_csv(&self, time: &chrono::DateTime<Local>) -> String {
        format!(
            "{},{},{},,",
            time.format("%+"),
            csv_field(&self.app_name),
            csv_field(&self.window_title)
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

static MOUSE_MOVED: AtomicBool = AtomicBool::new(false);

fn record_mouse_event(event: EventType) {
    if matches!(event, EventType::MouseMove { .. }) {
        MOUSE_MOVED.store(true, Ordering::Relaxed);
    }
}

/// Returns whether the mouse moved since the previous sample, then resets the flag.
pub(crate) fn take_mouse_movement() -> bool {
    MOUSE_MOVED.swap(false, Ordering::Relaxed)
}

/// Starts a background thread that records mouse movement without storing coordinates.
/// This function should be called once at program startup.
/// On Linux with Wayland, the user must be in the 'input' group for this to work.
pub fn start_mouse_listener() {
    thread::spawn(|| {
        if let Err(e) = listen(|event: Event| record_mouse_event(event.event_type)) {
            eprintln!("Warning: Failed to start mouse listener: {:?}", e);
            eprintln!("Mouse movement tracking may not work. On Linux, ensure you are in the 'input' group.");
        }
    });
}

pub(crate) fn get_state() -> State {
    match get_active_window() {
        Ok(window) => State {
            app_name: window.info.name,
            window_title: window.title,
        },
        Err(e) => {
            eprintln!("Error getting active window: {}", e);
            State {
                app_name: String::from(""),
                window_title: String::from(""),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_movement_once_per_sample_even_for_identical_deltas() {
        record_mouse_event(EventType::ButtonPress(rdev::Button::Left));
        assert!(!take_mouse_movement());

        // Identical fractional deltas must still count as movement in each interval.
        for _ in 0..2 {
            record_mouse_event(EventType::MouseMove { x: 0.25, y: 0.25 });
            assert!(take_mouse_movement());
            assert!(!take_mouse_movement());
        }

        // Multiple movements, including returning to the starting point, coalesce.
        record_mouse_event(EventType::MouseMove { x: 10.0, y: 20.0 });
        record_mouse_event(EventType::MouseMove { x: 0.0, y: 0.0 });
        assert!(take_mouse_movement());
        assert!(!take_mouse_movement());
    }

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
            };
            assert_eq!(
                state.to_csv(&time),
                format!("{},{encoded},{encoded},,", time.format("%+")),
                "input: {text:?}",
            );
        }
    }
}
