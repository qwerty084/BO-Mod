#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use process::open_process;
use std::{thread, time::Duration};
use tauri::Window;

use std::ffi::c_void;
use sysinfo::{Process, System};
use windows::Win32::{
    Foundation::*,
    System::{Diagnostics::Debug::*, ProcessStatus::EnumProcessModules},
};

mod process;

#[derive(Clone, serde::Serialize)]
struct Payload {
    timer: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
async fn init_process(window: Window) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let sleep_duration = Duration::from_millis(50);
    let mut target_pid: Option<u32> = None;
    let processes: Vec<&Process> = sys.processes_by_exact_name("BlackOps.exe").collect();

    for process in processes {
        target_pid = Some(process.pid().as_u32());
        break;
    }

    if target_pid.is_none() {
        return;
    }

    let process_handle_result = open_process(target_pid.unwrap());
    match process_handle_result {
        Ok(process_handle) => {
            let mut buffer: u32 = 0;
            let mut bytes_read: usize = 0;
            let mut modules: [HINSTANCE; 1024] = [HINSTANCE(0); 1024];
            let mut needed = 0;
            let modules_ptr = modules.as_mut_ptr() as *mut HMODULE;

            unsafe {
                EnumProcessModules(
                    process_handle,
                    modules_ptr,
                    std::mem::size_of_val(&modules) as u32,
                    &mut needed,
                )
                .ok();
            }

            let offset = 0x02EDA448;
            let base_address = modules[0].0 as usize;
            let timer_address = base_address + offset;

            loop {
                let read_success = unsafe {
                    ReadProcessMemory(
                        process_handle,
                        timer_address as *const _,
                        &mut buffer as *mut _ as *mut _,
                        std::mem::size_of::<u32>(),
                        Some(&mut bytes_read),
                    )
                };

                if read_success.is_ok() && bytes_read == std::mem::size_of::<u32>() {
                    let final_address = buffer + 0xC;
                    let mut game_timer_value: u32 = 0;
                    let read_final_value_success = unsafe {
                        ReadProcessMemory(
                            process_handle,
                            final_address as *const c_void,
                            &mut game_timer_value as *mut _ as *mut c_void,
                            std::mem::size_of::<u32>(),
                            Some(&mut bytes_read),
                        )
                    };

                    if read_final_value_success.is_ok() && bytes_read == std::mem::size_of::<u32>()
                    {
                        let seconds = game_timer_value / 1000;
                        let minutes = seconds / 60;
                        let hours = minutes / 60;

                        let seconds = seconds % 60;
                        let minutes = minutes % 60;

                        let _ = window.emit(
                            "timer",
                            Payload {
                                timer: format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
                            },
                        );
                    } else {
                        println!("Failed to read game timer value: {:?}", read_success.err());
                    }
                } else {
                    println!("Failed to read memory.");
                    break;
                }
                thread::sleep(sleep_duration);
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
