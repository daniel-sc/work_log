use active_win_pos_rs::get_active_window;
use device_query::{DeviceQuery, DeviceState};

#[derive(Debug, PartialEq)]
pub struct State {
    pub window_title: String,
    pub cursor: (i32, i32),
}

pub(crate) fn get_state() -> State {
    let mouse = DeviceState::new().get_mouse();
    let window_title = match get_active_window() {
        Ok(window) => format!("{}: {}", window.app_name, window.title),
        Err(e) => {
            eprintln!("Error getting active window: {:?}", e);
            String::new()
        }
    };
    State {
        window_title,
        cursor: mouse.coords,
    }
}
