use active_win_pos_rs::get_active_window;
use chrono::Local;
use device_query::{DeviceQuery, DeviceState};

// TODO react to keyboard input (e.g. typing)

#[derive(Debug, PartialEq)]
pub struct State {
    pub app_name: String,
    pub window_title: String,
    pub cursor: (i32, i32),
}

const SEPARATOR: &str = ",";

impl State {
    pub fn to_csv(&self, time: &chrono::DateTime<Local>,) -> String {
        format!("{}{}{}{}{}{}{}{}{}", time.format("%+"), SEPARATOR,self.app_name.replace(SEPARATOR, ""), SEPARATOR, self.window_title.replace(SEPARATOR, ""), SEPARATOR, self.cursor.0,SEPARATOR, self.cursor.1)
    }
}

pub(crate) fn get_state() -> State {
    let mouse = DeviceState::new().get_mouse();
    match get_active_window() {
        Ok(window) => State {
            app_name: window.app_name,
            window_title: window.title,
            cursor: mouse.coords,
        },
        Err(e) => {
            eprintln!("Error getting active window: {:?}", e);
            State {
                app_name: String::from(""),
                window_title: String::from(""),
                cursor: mouse.coords,
            }
        }
    }
}
