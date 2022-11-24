use rand::{seq::SliceRandom, Rng, SeedableRng};
use wasm_bindgen::prelude::*;

macro_rules! println {
    ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ALL_POSSIBLE: u16 = 0b11_1111_1110;
//                          98 7654 321

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Sudoku {
    board: [[u8; 9]; 9],
    possible: [[u16; 9]; 9],
}

impl Sudoku {
    const fn new() -> Self {
        Self {
            board: [[0; 9]; 9],
            possible: [[ALL_POSSIBLE; 9]; 9],
        }
    }
    fn get(&self, (x, y): Point) -> u8 {
        self.board[x][y]
    }
    fn set(&mut self, (x, y): Point, value: u8) {
        self.board[x][y] = value;
        for p in neighbor_iter((x, y)) {
            self.set_not_possible(p, value);
        }
    }
    fn set_tracking(&mut self, (x, y): Point, value: u8, changes: &mut Vec<Point>) {
        self.board[x][y] = value;
        for p in neighbor_iter((x, y)) {
            if self.is_possible(p, value) {
                self.set_not_possible(p, value);
                changes.push(p);
            }
        }
    }
    fn unset(&mut self, (x, y): Point) {
        let value = std::mem::replace(&mut self.board[x][y], 0);
        for p in neighbor_iter((x, y)) {
            if neighbor_iter(p).all(|n| self.get(n) != value) {
                self.possible[p.0][p.1] |= 1 << value;
            }
        }
    }

    fn is_possible(&self, (x, y): Point, val: u8) -> bool {
        (self.possible[x][y] >> val) & 1 == 1
    }
    fn set_not_possible(&mut self, (x, y): Point, val: u8) {
        self.possible[x][y] &= !(1 << val);
    }

    fn count_remaining(&self) -> usize {
        self.board.iter().flatten().filter(|&&x| x == 0).count()
    }
}

fn block_iter((x, y): Point) -> impl Iterator<Item = Point> {
    let bx = x / 3 * 3;
    let by = y / 3 * 3;
    (bx..bx + 3).flat_map(move |x| (by..by + 3).map(move |y| (x, y)))
}
fn row_iter(y: usize) -> impl Iterator<Item = Point> {
    (0..9).map(move |x| (x, y))
}
fn col_iter(x: usize) -> impl Iterator<Item = Point> {
    (0..9).map(move |y| (x, y))
}
fn neighbor_iter((x, y): Point) -> impl Iterator<Item = Point> {
    row_iter(y).chain(col_iter(x)).chain(block_iter((x, y)))
}

static mut SUDOKU: Sudoku = Sudoku::new();
static mut SEED: [u8; 32] = [0; 32];

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    println!("Hello world from Rust!");

    Ok(())
}

#[wasm_bindgen]
pub fn get(x: usize, y: usize) -> u8 {
    unsafe { SUDOKU.get((x, y)) }
}

pub fn is_possible(x: usize, y: usize, value: u8) -> bool {
    unsafe { SUDOKU.is_possible((x, y), value) }
}

#[derive(Debug, PartialEq, Eq)]
struct Cell {
    num_possible: u32,
    point: Point,
}
impl std::cmp::Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max heap, but we want the smallest num_possible to be at the top
        other.num_possible.cmp(&self.num_possible)
        // different points with the same num_possible are considered equal
    }
}
impl std::cmp::PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
type Queue = std::collections::BinaryHeap<Cell>;
fn queue_from_sudoku(sudoku: &Sudoku) -> Queue {
    let mut queue = Queue::new();
    for x in 0..9 {
        for y in 0..9 {
            if sudoku.get((x, y)) == 0 {
                let num_possible = sudoku.possible[x][y].count_ones();
                queue.push(Cell {
                    num_possible,
                    point: (x, y),
                });
            }
        }
    }
    queue
}

enum StepResult {
    Solved,
    NeedsGuess(Point),
    Impossible,
}

fn solve_step(state: &mut Sudoku, queue: &mut Queue) -> StepResult {
    let mut changes = vec![];

    while let Some(cell) = queue.pop() {
        if state.get(cell.point) != 0 {
            continue;
        }
        match cell.num_possible {
            0 => return StepResult::Impossible,
            1 => {
                let value = (1..=9).find(|&v| state.is_possible(cell.point, v)).unwrap();
                state.set_tracking(cell.point, value, &mut changes);
                for p in changes.drain(..) {
                    queue.push(Cell {
                        num_possible: state.possible[p.0][p.1].count_ones(),
                        point: p,
                    });
                }
            }
            _ => return StepResult::NeedsGuess(cell.point), // queue yields the smallest num_possible first
        }
    }
    StepResult::Solved
}

fn solve_recursive(mut state: Sudoku, rng: &mut impl Rng) -> Option<Sudoku> {
    let mut queue = queue_from_sudoku(&state);

    match solve_step(&mut state, &mut queue) {
        StepResult::Solved => Some(state),
        StepResult::Impossible => None,
        StepResult::NeedsGuess(point) => {
            let mut possible_values = (1..=9)
                .filter(|&v| state.is_possible(point, v))
                .collect::<Vec<_>>();
            possible_values.shuffle(rng);

            for value in possible_values {
                let mut new_state = state.clone();
                new_state.set(point, value);
                if let Some(solved) = solve_recursive(new_state, rng) {
                    return Some(solved);
                }
            }
            None
        }
    }
}

fn solve_non_guessing(state: &mut Sudoku) -> bool {
    let mut queue = queue_from_sudoku(state);

    matches!(solve_step(state, &mut queue), StepResult::Solved)
}

#[wasm_bindgen]
pub fn init(seed: &[u32]) {
    let mut rng = unsafe {
        for (bytes, input) in SEED.chunks_exact_mut(4).zip(seed) {
            bytes.copy_from_slice(&input.to_le_bytes());
        }
        rand::rngs::StdRng::from_seed(SEED)
    };

    let start_x = rng.gen_range(0..9);
    let start_y = rng.gen_range(0..9);
    let start_value = rng.gen_range(1..10);

    let mut sudoku = Sudoku::new();
    sudoku.set((start_x, start_y), start_value);

    if let Some(solved) = solve_recursive(sudoku, &mut rng) {
        unsafe {
            SUDOKU = solved;
        }
    } else {
        println!("Failed to generate sudoku");
    }
}

#[wasm_bindgen]
pub fn count_remaining() -> usize {
    unsafe { SUDOKU.count_remaining() }
}

#[wasm_bindgen]
pub fn solve() -> bool {
    let sudoku = unsafe { &mut SUDOKU };
    solve_non_guessing(sudoku)
}

#[wasm_bindgen]
pub fn reduce(difficulty: usize) {
    let to_remove = match difficulty {
        0 => return,
        1..=4 => 30 + difficulty * 6,
        _ => usize::MAX,
    };

    let mut rng = rand::rngs::StdRng::from_seed(unsafe { SEED });

    let mut remaining = (0..9)
        .flat_map(|x| (0..9).map(move |y| (x, y)))
        .collect::<Vec<_>>();
    remaining.shuffle(&mut rng);

    let sudoku = unsafe { &mut SUDOKU };

    let mut removed = 0;
    for p in remaining {
        let mut state = sudoku.clone();
        state.unset(p);
        if solve_non_guessing(&mut state) {
            sudoku.unset(p);
            removed += 1;
            if removed >= to_remove {
                return;
            }
        }
    }
}
