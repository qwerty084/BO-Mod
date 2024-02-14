use windows::Win32::{Foundation::*, System::Threading::*};

pub fn open_process(pid: u32) -> Result<HANDLE, &'static str> {
    unsafe {
        match OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid) {
            Ok(handle) => Ok(handle),
            Err(_) => Err("Failed to open process"),
        }
    }
}
