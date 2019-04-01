use super::queue::GetKey;
use count_possible;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
	pub x: u8,
	pub y: u8,
}

impl Point {
	pub fn new(x: u8, y: u8) -> Point {
		Point { x, y }
	}
}

impl GetKey for Point {
	type Key = usize;
	fn get_key(&self) -> usize {
		count_possible(self.x, self.y)
	}
}

use std::ops::*;

macro_rules! impl_opp {
	($type: tt) => {
		impl<'a> Mul<$type> for &'a Point {
			type Output = Point;
			fn mul(self, other: $type) -> Point {
				Point {
					x: self.x * other as u8,
					y: self.y * other as u8,
				}
			}
		}
		impl Mul<$type> for Point {
			type Output = Point;
			fn mul(self, other: $type) -> Point {
				Point {
					x: self.x * other as u8,
					y: self.y * other as u8,
				}
			}
		}
	};
}

impl_opp!(usize);
impl_opp!(u8);

