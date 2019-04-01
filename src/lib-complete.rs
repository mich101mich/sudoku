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

#[no_mangle]
pub unsafe fn get(x: u8, y: u8) -> u8 {
	SUDOKU[x as usize][y as usize]
}

#[no_mangle]
pub unsafe fn set(x: u8, y: u8, value: u8, changes: &mut Vec<(u8, u8)>) {
	SUDOKU[x as usize][y as usize] = value;
	changes.push((x, y));
	for (dx, dy) in Neighbours::of(x, y) {
		un_possible(dx, dy, value);
	}
}

unsafe fn refresh_possible() {
	for y in 0..9 {
		for x in 0..9 {
			if get(x, y) != 0 {
				continue;
			}
			POSSIBLE[x as usize][y as usize] = ALL_POSSIBLE;

			Neighbours::of(x, y)
				.map(|(mx, my)| get(mx, my))
				.filter(|v| *v != 0)
				.for_each(|v| un_possible(x, y, v));
		}
	}
}

unsafe fn un_possible(x: u8, y: u8, value: u8) {
	POSSIBLE[x as usize][y as usize] &= (1 << value) ^ ALL_POSSIBLE;
}

#[no_mangle]
pub unsafe fn is_possible(x: u8, y: u8, value: u8) -> bool {
	((POSSIBLE[x as usize][y as usize] >> value) & 1) == 1
}

unsafe fn get_values(x: u8, y: u8, my_points: &HashSet<(u8, u8, u8)>) -> Vec<u8> {
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

	let mut stack = vec![(vec![], HashSet::new(), 0)];

	let mut changes = vec![];
	let mut my_points = HashSet::new();

	let mut fail_count = 0;

	loop {
		'ok: while let Ok(change) = update(&mut changes) {
			if change {
				continue;
			}
			let remaining = count_remaining();
			if remaining == 0 {
				return;
			}

			let mut x;
			let mut y;
			let v;

			let mut tries = stack.last().expect("stack empty").2;

			if remaining > 30 {
				if tries > 500 {
					break 'ok;
				}

				let mut loop_count = 0;
				loop {
					loop_count += 1;
					if loop_count > 1000 {
						break 'ok;
					}
					x = rng.gen_range(0, 9);
					y = rng.gen_range(0, 9);
					if get(x, y) != 0 {
						continue;
					}
					let values = get_values(x, y, &my_points);
					v = if values.len() == 0 {
						continue;
					} else if values.len() == 1 {
						values[0]
					} else {
						values[rng.gen_range(0, values.len())]
					};
					break;
				}
			} else {
				let mut tx = 0;
				'search: loop {
					for ty in 0..9 {
						if get(tx, ty) != 0 {
							continue;
						}
						let values = get_values(tx, ty, &my_points);
						if values.len() == 0 {
							continue;
						}
						if tries > values.len() {
							tries -= values.len();
							continue;
						}
						v = if values.len() == 1 {
							values[0]
						} else {
							values[rng.gen_range(0, values.len())]
						};
						x = tx;
						y = ty;
						break 'search;
					}
					tx += 1;
					if tx == 9 {
						break 'ok;
					}
				}
			}
			my_points.insert((x, y, v));

			stack.push((changes, my_points, 0));
			changes = vec![];
			my_points = HashSet::new();
			set(x, y, v, &mut changes);
		}
		// err

		for (x, y) in changes.into_iter() {
			SUDOKU[x as usize][y as usize] = 0;
		}

		fail_count += 1;
		if fail_count > 1000 {
			return;
		}

		refresh_possible();
		if let Some(prev) = stack.pop() {
			changes = prev.0;
			my_points = prev.1;
		} else {
			return;
		}
		stack.last_mut().unwrap().2 += 1;
	}
}

unsafe fn only_possible(x: u8, y: u8) -> Option<u8> {
	let mut num = 0;
	for v in 1..10 {
		if is_possible(x, y, v) {
			if num != 0 {
				return None;
			}
			num = v;
		}
	}
	if num != 0 {
		Some(num)
	} else {
		None
	}
}

unsafe fn update(changes: &mut Vec<(u8, u8)>) -> Result<bool, ()> {
	let mut change = false;
	for y in 0..9 {
		for x in 0..9 {
			if get(x, y) != 0 {
				continue;
			}
			if POSSIBLE[x as usize][y as usize] == 0 {
				return Err(());
			}
			if let Some(num) = only_possible(x, y) {
				set(x, y, num, changes);
				change = true;
			}
		}
	}

	unsafe fn check_in<I: Iterator<Item = (u8, u8)> + Clone>(
		iter: I,
		changes: &mut Vec<(u8, u8)>,
	) -> Result<bool, ()> {
		let mut change = false;
		'values: for v in 1..10 {
			// check every value in this iter
			let mut pos = (10, 10);
			for (x, y) in iter.clone() {
				if get(x, y) == v {
					// if it is already placed, do nothing
					continue 'values;
				} else if get(x, y) == 0 && is_possible(x, y, v) {
					if pos != (10, 10) {
						// if one possibility was already found, do nothing
						continue 'values;
					}
					pos = (x, y);
				}
			}
			if pos == (10, 10) {
				// if no possibility was found, v cannot be placed
				return Err(());
			}
			// there was only on possibility => place
			set(pos.0, pos.1, v as u8, changes);
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
	for (bx, by) in BlockIter::at(0, 0).map(|(x, y)| (x * 3, y * 3)) {
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
			SUDOKU[x as usize][y as usize] = 0;
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
		SUDOKU[x as usize][y as usize] = 0;
	}
}
