use Point;

#[derive(Clone)]
pub struct RowIter {
	y: u8,
	i: u8,
}
impl RowIter {
	pub fn at(y: u8) -> RowIter {
		RowIter { y, i: 0 }
	}
}
impl Iterator for RowIter {
	type Item = Point;
	fn next(&mut self) -> Option<Point> {
		if self.i == 9 {
			return None;
		}
		let ret = Point::new(self.i, self.y);
		self.i += 1;
		Some(ret)
	}
}

#[test]
fn test_row_iter_1() {
	let mut iter = RowIter::at(0);
	for x in 0..9 {
		assert_eq!(iter.next(), Some(Point { x, y: 0 }));
	}
	assert_eq!(iter.next(), None);
}

#[test]
fn test_row_iter_2() {
	let mut iter = RowIter::at(8);
	for x in 0..9 {
		assert_eq!(iter.next(), Some(Point { x, y: 8 }));
	}
	assert_eq!(iter.next(), None);
}

#[test]
fn test_row_all_iter() {
	let mut been_there = [[0; 9]; 9];
	for ry in 0..9 {
		for Point { x, y } in RowIter::at(ry) {
			been_there[x as usize][y as usize] += 1;
		}
	}
	assert_eq!(been_there, [[1; 9]; 9]);
}
