use thiserror::Error;
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

use crate::messaging::error_log;

const PROCESS_START_OFFSET: u32 = 0x400000;

#[derive(Debug, PartialEq, Eq)]
pub struct Patch {
    pub offset: u32,
    pub original: Vec<u8>,
    pub patch: Vec<u8>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VerifyPatchError {
    #[error(
        "Patch is incompatible. Actual bytes at offset did not match expected bytes. Found {0:?}"
    )]
    Incompatible(Vec<u8>),
    #[error("Patch config contained original/patch of different lengths.")]
    MismatchedLengthsInConfig,
}

pub unsafe fn verify_patch(patch: &Patch) -> Result<(), VerifyPatchError> {
    if patch.original.len() != patch.patch.len() {
        return Err(VerifyPatchError::MismatchedLengthsInConfig);
    }
    let existing = {
        let mut buffer = vec![0 as u8; patch.original.len()];
        core::ptr::copy_nonoverlapping(
            (PROCESS_START_OFFSET + patch.offset) as *const u8,
            buffer.as_mut_ptr(),
            patch.original.len(),
        );
        buffer
    };
    if existing == patch.original {
        Ok(())
    } else {
        Err(VerifyPatchError::Incompatible(existing))
    }
}

/// It's extremely unsafe to call this without first calling `verify_patch` to
/// make sure the patch is valid.
/// Also, this is not thread-safe; race conditions could cause unsafe behavior if
/// two patches try to modify the same memory at once
pub unsafe fn apply_patch(patch: &Patch) -> Result<(), windows::core::Error> {
    let address = (PROCESS_START_OFFSET + patch.offset) as *mut u8;
    let mut old_protect = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(
        address as *mut _,
        patch.original.len(),
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    )?;
    core::ptr::copy_nonoverlapping(patch.patch.as_ptr(), address, patch.original.len());
    VirtualProtect(address as *mut _, 4, old_protect, &mut old_protect).unwrap_or_else(|err| {
        // This isn't really a big deal for our purposes, so we'll just log any failure and move on.
        error_log!(
            "Failed to restore memory protection for {:p}: {}",
            address,
            err
        );
    });
    Ok(())
}
