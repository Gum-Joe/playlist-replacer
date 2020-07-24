#[macro_use] extern crate log;
use text_io::read;
mod util;
use crate::util::logger;


fn main() {
    logger::setup_logger();
    info!("Playlist Replacer v{}", env!("CARGO_PKG_VERSION"));
    info!("Please enter the path, relative to /sdcard, to grab playlists from:");
    let path: String = read!("{}\n");
    info!("{}", path);
}
