mod args;
mod convert;
mod wage;
mod enumit;

use std::env;
use std::io::{self, Write};
use std::process;

use crate::args::Command;
use crate::wage::{Prefix, Unit, Wage};


fn main() {
	/// Run wageman with `std::env::args_os`, outputting to stdout.
	/// Error detail may be outputted to stderr.
	fn run() -> io::Result<()> {
		let args = env::args_os();

		let command = args::parse(args).map_err(
			|e| {
				eprintln!("{}", e.message);
				io::ErrorKind::InvalidInput
			}
		)?;

		match command {
			Command::Convert(args) => convert(args.wage),
			Command::Help(msg) | Command::Version(msg) =>
				writeln!(io::stdout(), "{}", msg)
		}
	}

	process::exit(
		match run() {
			Ok(()) => 0,
			Err(e) => match e.kind() {
				io::ErrorKind::InvalidInput => 1,
				io::ErrorKind::Interrupted => 130,
				_ => 255
			}
		}
	)
}


fn convert(wage: Wage) -> io::Result<()>{
	// Lock stdout to prevent repetitive locking.
	let stdout = io::stdout();
	let mut stdout = stdout.lock();

	let wages = convert::to_all(wage);

	for unit in Unit::iter() {
		writeln!(stdout, "{}:", unit)?;

		for prefix in Prefix::iter() {
			writeln!(stdout, "{}\t${}", prefix, wages[&unit][&prefix])?;
		}

		writeln!(stdout, "")?;
	}

	Ok(())
}
