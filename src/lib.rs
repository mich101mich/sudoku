use wasm_bindgen::prelude::*;
use web_sys::console;

use rand::{seq::SliceRandom, Rng, SeedableRng};

use std::collections::HashSet;

mod iters;
use iters::*;

macro_rules! println {
    ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ALL_POSSIBLE: u16 = 0b11_1111_1110;

static mut SUDOKU: [[u8; 9]; 9] = [[0; 9]; 9];
static mut POSSIBLE: [[u16; 9]; 9] = [[ALL_POSSIBLE; 9]; 9];
static mut SEED: [u8; 32] = [0; 32];

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    println!("Hello world from Rust!");

    Ok(())
}

#[wasm_bindgen]
pub fn get(x: usize, y: usize) -> u8 {
    unsafe { SUDOKU[x][y] }
}

pub fn set(x: usize, y: usize, value: u8, changes: &mut Vec<(usize, usize)>) {
    unsafe { SUDOKU[x][y] = value };
    changes.push((x, y));
    for (dx, dy) in Neighbours::of(x, y) {
        un_possible(dx, dy, value);
    }
}

fn refresh_possible() {
    for y in 0..9 {
        for x in 0..9 {
            if get(x, y) != 0 {
                continue;
            }
            unsafe { POSSIBLE[x][y] = ALL_POSSIBLE };

            Neighbours::of(x, y)
                .map(|(mx, my)| get(mx, my))
                .filter(|v| *v != 0)
                .for_each(|v| un_possible(x, y, v));
        }
    }
}

fn un_possible(x: usize, y: usize, value: u8) {
    unsafe { POSSIBLE[x][y] &= (1 << value) ^ ALL_POSSIBLE };
}

#[wasm_bindgen]
pub fn is_possible(x: usize, y: usize, value: u8) -> bool {
    ((unsafe { POSSIBLE[x][y] } >> value) & 1) == 1
}

fn none_possible(x: usize, y: usize) -> bool {
    unsafe { POSSIBLE[x][y] == 0 }
}

fn get_values(x: usize, y: usize, my_points: &HashSet<(usize, usize, u8)>) -> Vec<u8> {
    let mut values = vec![];
    for v in 1..10 {
        if is_possible(x, y, v) && !my_points.contains(&(x, y, v)) {
            values.push(v);
        }
    }
    values
}

#[wasm_bindgen]
pub fn init(seed: &[u32]) {
    let mut rng = unsafe {
        for (bytes, input) in SEED.chunks_exact_mut(4).zip(seed) {
            bytes.copy_from_slice(&input.to_le_bytes());
        }
        rand::rngs::StdRng::from_seed(SEED)
    };

    unsafe {
        SUDOKU = [[0; 9]; 9];
        POSSIBLE = [[ALL_POSSIBLE; 9]; 9];
    }

    {
        let start_x = rng.gen_range(0..9);
        let start_y = rng.gen_range(0..9);
        let start_value = rng.gen_range(1..10);
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
                    x = rng.gen_range(0..9);
                    y = rng.gen_range(0..9);
                    if get(x, y) != 0 {
                        continue;
                    }
                    let values = get_values(x, y, &my_points);
                    v = match values.choose(&mut rng) {
                        Some(v) => *v,
                        None => continue,
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
                        if values.is_empty() {
                            continue;
                        }
                        if tries > values.len() {
                            tries -= values.len();
                            continue;
                        }
                        v = *values.choose(&mut rng).unwrap();
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
            unsafe { SUDOKU[x][y] = 0 };
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

fn only_possible(x: usize, y: usize) -> Option<u8> {
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

fn update(changes: &mut Vec<(usize, usize)>) -> Result<bool, ()> {
    let mut change = false;
    for y in 0..9 {
        for x in 0..9 {
            if get(x, y) != 0 {
                continue;
            }
            if none_possible(x, y) {
                return Err(());
            }
            if let Some(num) = only_possible(x, y) {
                set(x, y, num, changes);
                change = true;
            }
        }
    }

    fn check_in<I: Iterator<Item = (usize, usize)> + Clone>(
        iter: I,
        changes: &mut Vec<(usize, usize)>,
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
            set(pos.0, pos.1, v, changes);
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

#[wasm_bindgen]
pub fn count_remaining() -> usize {
    unsafe { SUDOKU.iter() }
        .flatten()
        .filter(|&&x| x == 0)
        .count()
}

#[wasm_bindgen]
pub fn solve() -> bool {
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

#[wasm_bindgen]
pub fn reduce(difficulty: isize) {
    let mut rng = rand::rngs::StdRng::from_seed(unsafe { SEED });
    rng.gen_range(0..5);

    let mut removed: Vec<(usize, usize)> = vec![];
    let mut tries = 0;

    let mut _changes = vec![];

    while tries < difficulty {
        for &(x, y) in removed.iter() {
            unsafe { SUDOKU[x][y] = 0 };
        }

        let point = (rng.gen_range(0..9), rng.gen_range(0..9));
        let prev = get(point.0, point.1);
        if prev == 0 {
            continue;
        }
        unsafe { SUDOKU[point.0][point.1] = 0 };

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
        unsafe { SUDOKU[x][y] = 0 };
    }
}
