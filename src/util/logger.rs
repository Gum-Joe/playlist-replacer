extern crate simplelog;

use simplelog::*;

pub fn setup_logger() {
	CombinedLogger::init(
		vec![
			TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed),
		]
	).unwrap();
}
