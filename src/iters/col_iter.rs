#[derive(Clone)]
pub struct ColIter {
	x: usize,
	i: usize,
}
impl ColIter {
	pub fn at(x: usize) -> ColIter {
		ColIter { x, i: 0 }
	}
}
impl Iterator for ColIter {
	type Item = (usize, usize);
	fn next(&mut self) -> Option<(usize, usize)> {
		if self.i == 9 {
			return None;
		}
		let ret = (self.x, self.i);
		self.i += 1;
		Some(ret)
	}
}

#[test]
fn test_col_iter_1() {
	let mut iter = ColIter::at(0);
	for y in 0..9 {
		assert_eq!(iter.next(), Some((0, y)));
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_col_iter_2() {
	let mut iter = ColIter::at(5);
	for y in 0..9 {
		assert_eq!(iter.next(), Some((5, y)));
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_col_all_iter() {
	let mut been_there = [[0; 9]; 9];
	for cx in 0..9 {
		for (x, y) in ColIter::at(cx) {
			been_there[x as usize][y as usize] += 1;
		}
	}
	assert_eq!(been_there, [[1; 9]; 9]);
}
