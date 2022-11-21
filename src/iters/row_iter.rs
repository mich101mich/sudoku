#[derive(Clone)]
pub struct RowIter {
	y: usize,
	i: usize,
}
impl RowIter {
	pub fn at(y: usize) -> RowIter {
		RowIter { y, i: 0 }
	}
}
impl Iterator for RowIter {
	type Item = (usize, usize);
	fn next(&mut self) -> Option<(usize, usize)> {
		if self.i == 9 {
			return None;
		}
		let ret = (self.i, self.y);
		self.i += 1;
		Some(ret)
	}
}

#[test]
fn test_row_iter_1() {
	let mut iter = RowIter::at(0);
	for x in 0..9 {
		assert_eq!(iter.next(), Some((x, 0)));
	}
	assert_eq!(iter.next(), None);
}

#[test]
fn test_row_iter_2() {
	let mut iter = RowIter::at(8);
	for x in 0..9 {
		assert_eq!(iter.next(), Some((x, 8)));
	}
	assert_eq!(iter.next(), None);
}

#[test]
fn test_row_all_iter() {
	let mut been_there = [[0; 9]; 9];
	for ry in 0..9 {
		for (x, y) in RowIter::at(ry) {
			been_there[x as usize][y as usize] += 1;
		}
	}
	assert_eq!(been_there, [[1; 9]; 9]);
}
