use std::ffi::OsString;

use clap::{self, App, Arg, ArgMatches};
use clap::{crate_authors, crate_version, crate_name, crate_description};

use crate::wage::{Prefix, Unit, Wage};


#[derive(Debug)]
pub struct Args {
	pub wage: Wage
}

/// The action to be executed.
#[derive(Debug)]
pub enum Command {
	Help(String),
	Version(String),
	Convert(Args)
}

/// The error type for the argument parser. Contains only the error message.
#[derive(Debug)]
pub struct Error {
	pub message: String
}


/// Build clap's `App`. This specifies all arguments and metadata.
fn build_app() -> App<'static, 'static> {
	App::new(crate_name!())
		.about(crate_description!())
		.author(crate_authors!())
		.version(crate_version!())
		.template("{bin} {version}\nMade by {author}\n{about}\n\n{usage}\n\nFLAGS:\n{flags}")
		// Positional arguments:
		.arg(
			Arg::with_name("value")
				.required(true)
				.index(1)
		)
		// Unit prefix flags
		.arg(
			Arg::with_name("hour")
				.short("H")
				.help("Input wage for an hour")
				.conflicts_with_all(&[
					"day",
					"month"
				])
		)
		.arg(
			Arg::with_name("day")
				.short("d")
				.help("Input wage for a day")
				.conflicts_with_all(&[
					"month",
					"hour"
				])
		)
		.arg(
			Arg::with_name("month")
				.short("m")
				.help("Input wage for a month")
				.conflicts_with_all(&[
					"day",
					"hour"
				])
		)
		// Unit flag
		.arg(
			Arg::with_name("4 hours")
				.short("4")
				.help("Input wage for 4 hours")
				.conflicts_with_all(&[
					"6 hours",
					"8 hours"
				])
		)
		.arg(
			Arg::with_name("6 hours")
				.short("6")
				.help("Input wage for 6 hours")
				.conflicts_with_all(&[
					"4 hours",
					"8 hours"
				])
		)
		.arg(
			Arg::with_name("8 hours")
				.short("8")
				.help("Input wage for 8 hours")
				.conflicts_with_all(&[
					"4 hours",
					"6 hours"
				])
		)
}


/// Build an `Args` from clap's `ArgMatches`.
/// The matches are supposed to be valid, therefore there is no error handling/reporting.
fn build_args(args: ArgMatches) -> Result<Args, Error> {
	let value = args.value_of("value")
	                .expect("<value> not in ArgMatches") // value is required.
	                .trim()
	                .parse::<f64>()
	                .or(Err(Error { message: "Value must be an integer!".to_owned() }))?;

	let flag = |f| args.is_present(f);

	let prefix_flags = (
		flag("hour"),
		flag("day"),
		flag("month")
	);

	let prefix = match prefix_flags {
		(true, _, _) => Ok(Prefix::Hour),
		(_, true, _) => Ok(Prefix::Day),
		(_, _, true) => Ok(Prefix::Month),
		(_, _, _)    => Err(Error { message: "You must specify an prefix!".to_owned() }),
	}?;

	let unit_flags = (
		flag("4 hours"),
		flag("6 hours"),
		flag("8 hours")
	);

	let unit = match unit_flags {
		(true, _, _) => Ok(Unit::Hour4),
		(_, true, _) => Ok(Unit::Hour6),
		(_, _, true) => Ok(Unit::Hour8),
		(_, _, _)    => Err(Error { message: "You must specify an unit!".to_owned() }),
	}?;

	Ok(
		Args {
			wage: Wage {
				value: value,
				prefix: prefix,
				unit: unit
			}
		}
	)
}


/// Parse the arguments from `std::env::args_os`.
/// Returns the command to be executed, or the error message.
pub fn parse<
	T: Into<OsString> + Clone,
	A: IntoIterator<Item = T>
>(args: A) -> Result<Command, Error> {
	let app = build_app();

	match app.get_matches_from_safe(args) {
		Ok(arg_matches) => build_args(arg_matches).map(Command::Convert),
		Err(e) => match e.kind {
			clap::ErrorKind::HelpDisplayed    => Ok(Command::Help(e.message)),
			clap::ErrorKind::VersionDisplayed => Ok(Command::Version(e.message)),
			_ => Err(Error { message: e.message })
		}
	}
}
