#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use ical::parser::ical::component::IcalCalendar;
use serde::Serialize;
use std::fs::{self};
use std::path::Path;
use std::{fs::File, io::BufReader};
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
    return get_calendars_from_directory(Path::new("/home/arjun/.calendars"), 3);
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

fn main() {
    // parse_ics_file("/home/arjun/.calendars/arjun_local/arjunp0710@gmail.com/fe023000-3100-11ec-87ea-d5cae93c00a8.ics");
    let temp = get_calendars_from_directory(Path::new("/home/arjun/.calendars"), 3);
    println!("{:?}", temp.len());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_all_calendars])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
