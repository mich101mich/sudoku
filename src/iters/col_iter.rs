use Point;

#[derive(Clone)]
pub struct ColIter {
	x: u8,
	i: u8,
}
impl ColIter {
	pub fn at(x: u8) -> ColIter {
		ColIter { x, i: 0 }
	}
}
impl Iterator for ColIter {
	type Item = Point;
	fn next(&mut self) -> Option<Point> {
		if self.i == 9 {
			return None;
		}
		let ret = Point::new(self.x, self.i);
		self.i += 1;
		Some(ret)
	}
}

#[test]
fn test_col_iter_1() {
	let mut iter = ColIter::at(0);
	for y in 0..9 {
		assert_eq!(iter.next(), Some(Point { x: 0, y }));
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_col_iter_2() {
	let mut iter = ColIter::at(5);
	for y in 0..9 {
		assert_eq!(iter.next(), Some(Point { x: 5, y }));
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_col_all_iter() {
	let mut been_there = [[0; 9]; 9];
	for cx in 0..9 {
		for Point { x, y } in ColIter::at(cx) {
			been_there[x as usize][y as usize] += 1;
		}
	}
	assert_eq!(been_there, [[1; 9]; 9]);
}
