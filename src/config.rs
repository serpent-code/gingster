
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Config {
    pub verbose: bool,
    pub very_verbose: bool,
    pub use_emoji: bool,
}

impl Config {
	pub fn new() -> Self {
		let mut verbose = false;
		let mut very_verbose = false;
		let mut use_emoji = false;
		for argument in std::env::args() {
			match argument.as_str() {
				"-vv" | "--very-verbose" => {
					verbose = true;
					very_verbose = true;
				},
				"-v" | "--verbose" => very_verbose = true,
				"-e" | "--emoji" => use_emoji = true,
				_ => (),
			}
		}
		Self { verbose, very_verbose, use_emoji }
	}
}