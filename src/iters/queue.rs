pub trait GetKey {
	type Key: Ord;
	fn get_key(&self) -> <Self as GetKey>::Key;
}
impl<Key: Ord + Copy> GetKey for Key {
	type Key = Key;
	fn get_key(&self) -> Key {
		*self
	}
}

pub struct PriorityQueue<T: GetKey> {
	data: Vec<T>,
}

impl<T: GetKey> PriorityQueue<T> {
	pub fn new() -> PriorityQueue<T> {
		PriorityQueue { data: vec![] }
	}
	pub fn insert(&mut self, element: T) {
		ordered_insert(&mut self.data, element, |e| e.get_key())
	}
	pub fn insert_all<I: Iterator<Item=T>>(&mut self, iter: I) {
		for e in iter {
			self.insert(e);
		}
	}
	pub fn pop(&mut self) -> Option<T> {
		self.data.pop()
	}
	pub fn len(&self) -> usize {
		self.data.len()
	}
	pub fn remove_by<F: Fn(&T) -> bool>(&mut self, f: F) {
		self.data.retain(|e| !f(e));
	}
	pub fn update(&mut self) {
		if (self.len() == 0) {
			return;
		}
		let mut reinsert = vec![];
		let mut cur_key = self.data[0].get_key();
		use std::cmp::Ordering::*;
		let mut i = 1;
		while i < self.len() {
			let key = self.data[i].get_key();
			// next key <= cur_key  because "smallest" element in last position
			match key.cmp(&cur_key) {
				Less => cur_key = key,
				Equal => {}
				Greater => {
					reinsert.push(self.data.remove(i));
					i -= 1;
				}
			};
			i += 1;
		}
		for e in reinsert {
			self.insert(e);
		}
	}
	pub fn iter(&self) -> std::slice::Iter<T> {
		self.data.iter()
	}
}

use std::ops::Index;

impl<T: GetKey> Index<usize> for PriorityQueue<T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.data[i]
	}
}

pub struct QueueDrain<T: GetKey> {
	queue: PriorityQueue<T>,
}
impl<T: GetKey> Iterator for QueueDrain<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.queue.pop()
	}
}

impl<T: GetKey> IntoIterator for PriorityQueue<T> {
	type Item = T;
	type IntoIter = QueueDrain<T>;
	fn into_iter(self) -> QueueDrain<T> {
		QueueDrain { queue: self }
	}
}

fn ordered_insert<T, V, F>(vector: &mut Vec<T>, element: T, get_value: F)
where
	V: Ord,
	F: Fn(&T) -> V,
{
	let value = get_value(&element);
	let pos = vector
		.binary_search_by(|x| value.cmp(&get_value(x)))
		.unwrap_or_else(|pos| pos);
	vector.insert(pos, element);
}
