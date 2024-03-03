use sysinfo::{Process, System};
use windows::Win32::{Foundation::*, System::Threading::*};

/// Opens a process with the specified process ID (pid) and returns a handle to the process.
/// Returns `Some(handle)` if the process is successfully opened, or `None` if an error occurs.
pub fn open_process(pid: u32) -> Option<HANDLE> {
    unsafe {
        match OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid) {
            Ok(handle) => Some(handle),
            Err(_) => None,
        }
    }
}

/// Retrieves the process ID (pid) of the first process with the name "BlackOps.exe".
/// Returns `Some(pid)` if a matching process is found, or `None` if no matching process is found.
pub fn get_pid() -> Option<u32> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let processes: Vec<&Process> = sys.processes_by_exact_name("BlackOps.exe").collect();

    match processes.first() {
        Some(pid) => Some(pid.pid().as_u32()),
        None => None,
    }
}
