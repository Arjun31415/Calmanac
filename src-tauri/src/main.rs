#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use directories::ProjectDirs;
use ical::parser::ical::component::IcalCalendar;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Debug, Deserialize, Clone)]
struct Config {
    calendars_path: String,
    depth: u8,
}
static CONFIG: Lazy<Config> = Lazy::new(|| get_config());
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct CalmanacCalendar(IcalCalendar);
impl CalmanacCalendar {
    fn new(calendar: IcalCalendar) -> CalmanacCalendar {
        CalmanacCalendar(calendar)
    }
}

#[tauri::command]
fn get_all_calendars() -> Vec<CalmanacCalendar> {
    let calendars_path = CONFIG.calendars_path.clone();

    return get_calendars_from_directory(Path::new(&calendars_path), 3);
}
// given a path to .ics file, parse it
fn parse_ics_file(path: &str) -> CalmanacCalendar {
    let buf = BufReader::new(File::open(path).unwrap());
    let reader = ical::IcalParser::new(buf);
    let mut calendar: CalmanacCalendar = CalmanacCalendar::new(IcalCalendar::new());
    for component in reader {
        // add the component to the calendar
        calendar = CalmanacCalendar(component.unwrap());
    }
    // print!("{:?}", calendar.events[0].properties);
    calendar
}

/// given a path to a directory, recursively (till max_depth is reached)
///  search for .ics files
/// and parse them into a vector of CalmanacCalendar
fn get_calendars_from_directory(dir: &Path, max_depth: i32) -> Vec<CalmanacCalendar> {
    if max_depth <= 0 {
        return Vec::new();
    }
    let mut cal = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let other_cal = get_calendars_from_directory(&path, max_depth - 1);
                cal.extend(other_cal);
                // append other_cal to calendars
            } else {
                if path.extension().unwrap() == "ics" {
                    cal.push(parse_ics_file(path.to_str().unwrap()));
                }
            }
        }
    }
    cal
}
fn get_config_dir() -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "calmanac", "calmanac").unwrap();
    let config_dir = project_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).unwrap();
    }
    config_dir.to_path_buf()
}
fn get_config_file_path() -> String {
    let config_dir = get_config_dir();
    let config_file_path = config_dir.join("config.toml");
    config_file_path.to_str().unwrap().to_string()
}
fn get_config() -> Config {
    let config_file_path = get_config_file_path();
    dbg!(&config_file_path);
    let config_str = fs::read_to_string(config_file_path).unwrap_or("".to_string());
    let config: Config = toml::from_str(&config_str).unwrap_or(Config {
        // todo give default value
        calendars_path: get_config_dir()
            .join(Path::new("calendars"))
            .to_str()
            .unwrap()
            .to_string(),
        depth: 5,
    });
    config
}

fn main() {
    let calendars_path = CONFIG.calendars_path.clone();
    dbg!(&CONFIG);
    // parse_ics_file("/home/arjun/.calendars/arjun_local/arjunp0710@gmail.com/fe023000-3100-11ec-87ea-d5cae93c00a8.ics");
    // let temp = get_all_calendars();
    // println!("{:?}", temp.len());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_calendars])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
