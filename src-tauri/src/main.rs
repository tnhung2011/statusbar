// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use sys_locale::get_locale;

//use serde::ser::Serialize;

use winapi::shared::windef::RECT;
use winapi::ctypes::c_void;
use winapi::um::winuser::SystemParametersInfoA;
use winapi::um::winuser::GetSystemMetrics;

#[derive(serde::Serialize)]
struct TaskbarHeight {
    w: i32,
    h: i32,
    aty: i32
}

/* #[tauri::command]
fn appdat() -> String {
    let appdata = env::var("APPDATA");
    return appdata.expect("[StatusBar] parrots encountered while snatching %APPDATA%");
} */

#[tauri::command]
fn tbdim() -> String {
    let mut r = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0
    };
    let ptr: *mut RECT = &mut r;
    unsafe { SystemParametersInfoA(0x0030, 0, ptr as *mut c_void, 0) }; // SPI_GETWORKAREA

    let scr_h = unsafe { GetSystemMetrics(1) }; // SM_CYSCREEN
    let bl: bool = (r.bottom - scr_h) != 0;

    let tb = TaskbarHeight {
        aty: (r.bottom) * (bl as i32),
        h:   (r.bottom - scr_h) + r.top,
        w:   unsafe { GetSystemMetrics(0) } // SM_CXSCREEN
    };

    return serde_json::to_string(&tb).unwrap();
}

#[tauri::command]
fn localise() -> String {
    return match get_locale() {
        Some(v) => v,
        None => "en-US".to_string(),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            localise, tbdim
        ])
        .run(tauri::generate_context!())
        .expect("[StatusBar] parrots encountered while running");
}