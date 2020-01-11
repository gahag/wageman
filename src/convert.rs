use std::collections::HashMap;

use crate::wage::{Prefix, Unit, Wage};


pub fn to_all(wage: Wage) -> HashMap<Unit, HashMap<Prefix, f64>> {
	let wage = Wage {
		prefix: Prefix::Hour, // Convert to hour.
		unit: Unit::Hour4, // Convert to 4 hours.
		value: wage.value / wage.prefix.to_hours(wage.unit) as f64
	};

	let mut result = HashMap::new();

	for unit in Unit::iter() {
		let mut wages = HashMap::new();

		for prefix in Prefix::iter() {
			wages.insert(
				prefix,
				wage.value * prefix.to_hours(unit) as f64
			);
		}

		result.insert(unit, wages);
	}

	result
}
