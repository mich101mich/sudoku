#![allow(unused)]

extern crate rand;
use rand::{ChaChaRng, Rng, SeedableRng};

use std::collections::HashSet;

mod rust_src;
use rust_src::*;

const ALL_POSSIBLE: u16 = 0b1111111110;

static mut SUDOKU: [[u8; 9]; 9] = [[0; 9]; 9];
static mut POSSIBLE: [[u16; 9]; 9] = [[ALL_POSSIBLE; 9]; 9];
static mut SEED: [u8; 32] = [0; 32];

fn get_sudoku(pos: Point) -> &'static mut u8 {
	unsafe { &mut SUDOKU[pos.x as usize][pos.y as usize] }
}

#[no_mangle]
pub fn get(x: u8, y: u8) -> u8 {
	*get_sudoku(Point::new(x, y))
}

#[no_mangle]
pub fn set(x: u8, y: u8, value: u8, changes: &mut Vec<Point>) {
	let pos = Point::new(x, y);
	*get_sudoku(pos) = value;
	changes.push(pos);
	for dpos in Neighbours::of(pos) {
		un_possible(dpos.x, dpos.y, value);
	}
}

fn refresh_possible() {
	for y in 0..9 {
		for x in 0..9 {
			if get(x, y) != 0 {
				continue;
			}
			*get_possible(x, y) = ALL_POSSIBLE;

			Neighbours::of(Point::new(x, y))
				.map(|pos| *get_sudoku(pos))
				.filter(|v| *v != 0)
				.for_each(|v| un_possible(x, y, v));
		}
	}
}

fn get_possible(x: u8, y: u8) -> &'static mut u16 {
	unsafe { &mut POSSIBLE[x as usize][y as usize] }
}

fn un_possible(x: u8, y: u8, value: u8) {
	*get_possible(x, y) &= (1 << value) ^ ALL_POSSIBLE;
}

#[no_mangle]
pub fn is_possible(x: u8, y: u8, value: u8) -> bool {
	((*get_possible(x, y) >> value) & 1) == 1
}

fn get_values(x: u8, y: u8, my_points: &HashSet<(u8, u8, u8)>) -> Vec<u8> {
	let mut values = vec![];
	for v in 1..10 {
		if is_possible(x, y, v) && !my_points.contains(&(x, y, v)) {
			values.push(v);
		}
	}
	values
}

#[no_mangle]
pub unsafe fn set_seed(n: usize, seed: u8) {
	SEED[n] = seed;
}

#[no_mangle]
pub unsafe fn get_seed(n: usize) -> u8 {
	SEED[n]
}

fn count_possible(x: u8, y: u8) -> usize {
	get_possible(x, y).count_ones() as usize
}

#[no_mangle]
pub unsafe fn init() {
	let mut rng = rand::ChaChaRng::from_seed(SEED);

	SUDOKU = [[0; 9]; 9];
	POSSIBLE = [[ALL_POSSIBLE; 9]; 9];

	{
		let start_x = rng.gen_range(0, 9);
		let start_y = rng.gen_range(0, 9);
		let start_value = rng.gen_range(1, 10);
		set(start_x, start_y, start_value, &mut vec![]);
	}

	let mut bins = vec![];
	for i in 0..9 {
		bins.push(vec![]);
	}
	for x in 0..9 {
		for y in 0..9 {
			if get(x, y) == 0 {
				bins[count_possible(x, y) - 1].push(Point::new(x, y));
			}
		}
	}
	let mut priority_queue = PriorityQueue::<Point>::new();
	for bin in bins {
		for i in 0..bin.len() {
			bin.swap(i, rng.gen_range(0, bin.len()));
		}
		priority_queue.insert_all(bin.into_iter());
	}
	if !solve_step(priority_queue) {
		panic!("how?!?");
	}
}

fn solve_step(mut queue: PriorityQueue<Point>) -> bool {
	if let Some(p) = queue.pop() {
		for i in 1..10 {
			let mut changes = vec![];
			set(p.x, p.y, i, &mut changes);
			match update(&mut queue, &mut changes) {}
		}
		false
	} else {
		true // all points placed
	}
}

fn update(queue: &mut PriorityQueue<Point>, changes: &mut Vec<Point>) -> Result<bool, ()> {
	let mut change = false;

	let mut to_visit = HashSet::new();
	for e in queue.iter() {
		match count_possible(e.x, e.y) {
			0 => return Err(()),
			1 => {
				to_visit.insert(*e);
			}
			_ => {}
		}
	}
	while let Some(pos) = to_visit.iter().next().and_then(|e| to_visit.take(e)) {
		match count_possible(pos.x, pos.y) {
			0 => return Err(()),
			1 => {
				let mut v = 0;
				for i in 1..10 {
					if is_possible(pos.x, pos.y, i) {
						v = i;
						break;
					}
				}
				for e in Neighbours::of(pos).filter(|&p| *get_sudoku(p) == 0) {
					to_visit.insert(e);
				}
			}
			_ => {}
		}
	}

	unsafe fn check_in<I: Iterator<Item = Point> + Clone>(
		iter: I,
		changes: &mut Vec<Point>,
	) -> Result<bool, ()> {
		let mut change = false;
		'values: for v in 1..10 {
			// check every value in this iter
			let mut pos = Point::new(10, 10);
			for p in iter.clone() {
				if *get_sudoku(p) == v {
					// if it is already placed, do nothing
					continue 'values;
				} else if *get_sudoku(p) == 0 && is_possible(p.x, p.y, v) {
					if pos.x != 10 && pos.y != 10 {
						// if one possibility was already found, do nothing
						continue 'values;
					}
					pos = p;
				}
			}
			if pos.x == 10 && pos.y == 10 {
				// if no possibility was found, v cannot be placed
				return Err(());
			}
			// there was only on possibility => place
			set(pos.x, pos.y, v as u8, changes);
			change = true;
		}
		Ok(change)
	}

	for y in 0..9 {
		match check_in(RowIter::at(y), changes) {
			Ok(chg) => change = change || chg,
			Err(()) => return Err(()),
		}
	}
	for x in 0..9 {
		match check_in(ColIter::at(x), changes) {
			Ok(chg) => change = change || chg,
			Err(()) => return Err(()),
		}
	}
	for (bx, by) in BlockIter::at(Point::new(0, 0)).map(|(x, y)| (x * 3, y * 3)) {
		match check_in(BlockIter::at(bx, by), changes) {
			Ok(chg) => change = change || chg,
			Err(()) => return Err(()),
		}
	}

	Ok(change)
}

#[no_mangle]
pub unsafe fn count_remaining() -> u32 {
	let mut count = 0;
	for y in 0..9 {
		for x in 0..9 {
			if SUDOKU[x][y] == 0 {
				count += 1;
			}
		}
	}
	count
}

#[no_mangle]
pub unsafe fn solve() -> bool {
	refresh_possible();
	let mut changes = vec![];
	while let Ok(change) = update(&mut changes) {
		if change {
			continue;
		}
		let remaining = count_remaining();
		return remaining == 0;
	}
	false
}

#[no_mangle]
pub unsafe fn reduce(difficulty: i32) {
	let mut rng = rand::ChaChaRng::from_seed(SEED);
	rng.gen_range(0, 5);

	let mut removed = vec![];
	let mut tries = 0;

	let mut _changes = vec![];

	while tries < difficulty {
		for &(x, y) in removed.iter() {
			*get_sudoku(x, y) = 0;
		}

		let point = (rng.gen_range(0, 9), rng.gen_range(0, 9));
		let prev = get(point.0, point.1);
		if prev == 0 {
			continue;
		}
		SUDOKU[point.0 as usize][point.1 as usize] = 0;

		if solve() {
			removed.push(point);
			tries = 0;
		} else {
			tries += 1;
			set(point.0, point.1, prev, &mut _changes);
			if !solve() {
				panic!();
			}
		}
	}

	for &(x, y) in removed.iter() {
		*get_sudoku(x, y) = 0;
	}
}
