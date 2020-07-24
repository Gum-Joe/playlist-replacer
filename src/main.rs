#[macro_use] extern crate log;
use text_io::read;
use std::process::{Command};
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};
mod util;
use crate::util::logger;

fn fetch_playerlists_from_sdcard(directory_as_string: &str) -> Result<Vec<u8>, String> {
    info!("Grabbing list of .m3u files from /sdcard/{}", directory_as_string);
    let formatted_path = format!("/sdcard/{}/*.m3u", directory_as_string);

    if formatted_path.contains("..") {
        return Err("Path contained a '..'! This is not allowed!".to_string());
    }

    debug!("Running adb....");
    let output = Command::new("adb")
        .arg("shell")
        .arg("ls")
        .arg(formatted_path)
        .output()
        .expect("Could not execute adb command!");
    
    return Ok(output.stdout);
}


fn main() {
    logger::setup_logger();
    info!("Playlist Replacer v{}", env!("CARGO_PKG_VERSION"));
    info!("Please enter the path, relative to /sdcard, to grab playlists from:");
    let path: String = read!("{}\r\n");
    let output = fetch_playerlists_from_sdcard(&path).unwrap();
    let output_string = match std::str::from_utf8(&output) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let split_output = output_string.split("\n");
    let out_collected: Vec<&str> = split_output.collect();

    let mut table = Table::new();
    table.add_row(row!["ID", "Name", "Path"]);
    for i in 0..out_collected.len() {
        table.add_row(row![i, out_collected[i]]);
    }
    table.printstd();

    info!("Please select a playlist from above by ID:");
    let playlist_number: u32 = read!("{}\r\n");
    debug!("{}", playlist_number);

}
