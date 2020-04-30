pub struct ForEachExtend<'a, T> {
	vec: &'a mut Vec<T>,
	pos: usize,
}

impl<'a, T> ForEachExtend<'a, T> {
	pub fn push(&mut self, t: T) {
		self.vec.push(t);
	}

	pub fn current(&self) -> &T {
		&self.vec[self.pos]
	}

	pub fn current_mut(&mut self) -> &mut T {
		&mut self.vec[self.pos]
	}

	pub fn index(&self) -> usize {
		self.pos
	}
}

pub trait ForEachExtendTrait<T> {
	fn for_each_extend<F: for<'a> FnMut(&mut ForEachExtend<'a, T>)>(&mut self, f: F) -> &mut Self;
}

impl<T> ForEachExtendTrait<T> for Vec<T> {
	fn for_each_extend<F: for<'a> FnMut(&mut ForEachExtend<'a, T>)>(&mut self, mut f: F) -> &mut Self {
		let mut for_each_extend = ForEachExtend {
			vec: self,
			pos: 0,
		};
		while for_each_extend.pos < for_each_extend.vec.len() {
			f(&mut for_each_extend);
			for_each_extend.pos += 1;
		}
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let mut a: Vec<i32> = Vec::new();
		let mut is_lambda_called = false;
		a.for_each_extend(|_| {
			is_lambda_called = true;
		});

		assert!(!is_lambda_called);
		assert_eq!(a, []);
	}

	#[test]
	fn it_works() {
		let mut a = vec![1, 2, 3, 4];
		a.for_each_extend(|x| {
			if *x.current() == 3 {
				x.push(5);
				x.push(6);
				x.push(7);
				x.push(8);
			}
		});

		assert_eq!(a, [1, 2, 3, 4, 5, 6, 7, 8]);
	}

	#[test]
	fn complex_example() {
		let mut a = vec![1, 2, 3, 4, 5, 6];
		a.for_each_extend(|x| {
			if x.current() % 3 == 0 {
				x.push(x.current() + 1);
				*x.current_mut() = 10 * x.index();
			}
		}).iter_mut()
			.for_each(|x| *x += 1);

		assert_eq!(a, [2, 3, 21, 5, 6, 51, 5, 8]);
	}
}
