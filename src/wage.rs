use std::fmt;

use crate::enumit::EnumIter;


/// The decimal unit prefix of wage.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Prefix {
	Hour,
	Day,
	Month
}

/// The unit of wage, i.e. the workday's length.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Unit {
	Hour4,
	Hour6,
	Hour8
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Wage {
	pub value: f64,
	pub prefix: Prefix,
	pub unit: Unit
}


impl fmt::Display for Prefix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(
			match self {
				Prefix::Hour  => "Hour",
				Prefix::Day   => "Day",
				Prefix::Month => "Month"
			}
		)
	}
}

impl Prefix {
	pub fn to_hours(self, unit: Unit) -> u32 {
		match self {
			Prefix::Hour  => 1,
			Prefix::Day   => unit.value(),
			Prefix::Month => unit.value() * 30
		}
	}

	pub fn iter() -> EnumIter<Prefix> {
		static VARIANTS: &'static [Prefix] = &[
			Prefix::Hour,
			Prefix::Day,
			Prefix::Month
		];

		EnumIter::new(VARIANTS)
	}
}


impl fmt::Display for Unit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(
			match self {
				Unit::Hour4 => "4 hours",
				Unit::Hour6 => "6 hours",
				Unit::Hour8 => "8 hours"
			}
		)
	}
}

impl Unit {
	pub fn value(self) -> u32 {
		match self {
			Unit::Hour4 => 4,
			Unit::Hour6 => 6,
			Unit::Hour8 => 8
		}
	}

	pub fn iter() -> EnumIter<Unit> {
		static VARIANTS: &'static [Unit] = &[
			Unit::Hour4,
			Unit::Hour6,
			Unit::Hour8
		];

		EnumIter::new(VARIANTS)
	}
}
