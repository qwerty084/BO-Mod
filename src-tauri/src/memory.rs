use std::ffi::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

/// Reads an unsigned integer from the specified process memory.
/// # Arguments
/// * `handle` - The handle to the process.
/// * `address` - The address in the process memory to read from.
/// # Returns
/// Returns `Some(u32)` if the read operation was successful and the value was read correctly.
/// Returns `None` if the read operation failed or the number of bytes read does not match the size of `u32`.
pub fn read_unsigned_integer(handle: HANDLE, address: *const c_void) -> Option<u32> {
    let mut bytes_read: usize = 0;
    let mut value: u32 = 0;

    let read_final_value_success = unsafe {
        ReadProcessMemory(
            handle,
            address,
            &mut value as *mut _ as *mut _,
            std::mem::size_of::<u32>(),
            Some(&mut bytes_read),
        )
    };

    if read_final_value_success.is_ok() && bytes_read == std::mem::size_of::<u32>() {
        Some(value)
    } else {
        None
    }
}
