use std::ffi::CString;

use windows::Win32::Foundation::{BOOL, FALSE, HINSTANCE, TRUE};

use crate::messaging::show_message_box;

mod apply_patch;
mod init;
mod messaging;
mod patches;
mod patches_old;

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _dll_module: HINSTANCE,
    call_reason: u32,
    _reserved: isize,
) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;

    match call_reason {
        DLL_PROCESS_ATTACH => match init::init() {
            Ok(_) => TRUE,
            Err(err) => {
                unsafe {
                    let message = CString::new(err.to_string()).unwrap();
                    show_message_box(
                        "QuickStart - Error patching game",
                        &message.to_string_lossy(),
                    );
                }
                FALSE
            }
        },
        _ => TRUE,
    }
}
