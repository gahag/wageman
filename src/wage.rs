use std::fmt;
use std::iter::Peekable;

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

#[derive(Debug, Clone)]
pub struct WageIter {
	base: Wage,
	prefix: EnumIter<Prefix>,
	unit: Peekable<EnumIter<Unit>>
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


impl Iterator for WageIter {
	type Item = Wage;

	fn next(&mut self) -> Option<Self::Item> {
		let prefix = match self.prefix.next() {
			Some(p) => p,
			None => {
				self.unit.next();
				self.prefix = Prefix::iter();
				self.prefix.next().expect("prefix enum iter empty")
			}
		};

		let unit = self.unit.peek()?.clone();

		return Some(
			Wage {
				prefix: prefix,
				unit: unit,
				value: self.base.value * prefix.to_hours(unit) as f64
			}
		)
	}
}

impl Wage {
	pub fn variations(self) -> WageIter {
		WageIter {
			base: Wage {
				prefix: Prefix::iter()
				               .next()
				               .expect("prefix enum iter empty"),

				unit: Unit::iter()
				           .next()
				           .expect("unit enum iter empty"),

				value: self.value / self.prefix.to_hours(self.unit) as f64
			},
			prefix: Prefix::iter(),
			unit: Unit::iter().peekable()
		}
	}
}
