use BlockIter;
use ColIter;
use Point;
use RowIter;

use std::iter::Map;
use std::ops::Range;

pub struct Neighbours {
	pos: Point,
	col: ColIter,
	row: RowIter,
	block: BlockIter,
}
impl Neighbours {
	pub fn of(pos: Point) -> Neighbours {
		Neighbours {
			pos,
			col: ColIter::at(pos.x),
			row: RowIter::at(pos.y),
			block: BlockIter::at(pos),
		}
	}
}
impl Iterator for Neighbours {
	type Item = Point;
	fn next(&mut self) -> Option<Point> {
		let ret = self
			.col
			.next()
			.or_else(|| self.row.next())
			.or_else(|| self.block.next());

		if ret == Some(self.pos) {
			self.next()
		} else {
			ret
		}
	}
}

#[test]
fn test_neighbours() {
	let mut iter = Neighbours::of(Point::new(7, 3));
	for y in 0..3 {
		assert_eq!(iter.next(), Some(Point { x: 7, y }));
	}
	for y in 4..9 {
		assert_eq!(iter.next(), Some(Point { x: 7, y }));
	}
	for x in 0..7 {
		assert_eq!(iter.next(), Some(Point { x, y: 3 }));
	}
	for x in 8..9 {
		assert_eq!(iter.next(), Some(Point { x, y: 3 }));
	}
	assert_eq!(iter.next(), Some(Point { x: 6, y: 3 }));
	assert_eq!(iter.next(), Some(Point { x: 8, y: 3 }));
	for y in 4..6 {
		for x in 6..9 {
			assert_eq!(iter.next(), Some(Point { x, y }));
		}
	}
	assert_eq!(iter.next(), None);
}
