#[derive(Clone)]
pub struct BlockIter {
	x: usize,
	y: usize,
	dx: usize,
	dy: usize,
}
impl BlockIter {
	pub fn at(x: usize, y: usize) -> BlockIter {
		BlockIter {
			x: x / 3 * 3,
			y: y / 3 * 3,
			dx: 0,
			dy: 0,
		}
	}
}
impl Iterator for BlockIter {
	type Item = (usize, usize);
	fn next(&mut self) -> Option<(usize, usize)> {
		if self.dy == 3 {
			return None;
		}
		let ret = (self.x + self.dx, self.y + self.dy);
		self.dx += 1;
		if self.dx == 3 {
			self.dy += 1;
			self.dx = 0;
		}
		Some(ret)
	}
}

#[test]
fn test_block_iter_1() {
	let mut iter = BlockIter::at(0, 0);
	for y in 0..3 {
		for x in 0..3 {
			assert_eq!(iter.next(), Some((x, y)));
		}
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_block_iter_2() {
	let mut iter = BlockIter::at(5, 8);
	for y in 6..9 {
		for x in 3..6 {
			assert_eq!(iter.next(), Some((x, y)));
		}
	}
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn test_block_all_iter() {
	let mut been_there = [[0; 9]; 9];
	for (bx, by) in BlockIter::at(0, 0).map(|(x, y)| (x * 3, y * 3)) {
		for (x, y) in BlockIter::at(bx, by) {
			been_there[x as usize][y as usize] += 1;
		}
	}
	assert_eq!(been_there, [[1; 9]; 9]);
}
