pub struct EnumIter<T: 'static + Copy>(
	std::iter::Copied<std::slice::Iter<'static, T>>
);


impl<T: Copy> EnumIter<T> {
	pub fn new(values: &'static [T]) -> EnumIter<T> {
		EnumIter(
			values.iter()
			      .copied()
		)
	}
}


impl<T: Copy> Iterator for EnumIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}
