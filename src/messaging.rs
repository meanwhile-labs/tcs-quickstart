macro_rules! error_log {
	($($arg:tt)*) => {
        use crate::messaging::error_log_str;
        error_log_str(&format!($($arg)*));
	}
}
use std::ffi::CString;

use chrono::Local;
pub(crate) use error_log;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE},
    },
};

pub unsafe fn show_message_box(title: &str, message: &str) {
    let title = CString::new(title).unwrap();
    let message = CString::new(message).unwrap();
    MessageBoxA(
        HWND::default(),
        PCSTR::from_raw(message.as_ptr() as *const u8),
        PCSTR::from_raw(title.as_ptr() as *const u8),
        MESSAGEBOX_STYLE(0x00000000),
    );
}

pub fn error_log_str(message: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    let timestamp = Local::now();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("quickstart-log.txt");
    let written =
        file.and_then(|mut file| writeln!(file, "[{}] {}", timestamp.to_string(), message));

    if let Err(_err) = written {
        unsafe {
            show_message_box(
                "QuickStart mod error",
                &("Unable to write to quickstart-log.txt; reporting error here instead.\n"
                    .to_owned()
                    + message),
            );
        }
    }
}
