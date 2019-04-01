
export default class Wasm {
	/**
	 * gets the Sudoku value at (x, y)
	 * @param x the x coordinate
	 * @param y the y coordinate
	 */
	static get(x: number, y: number) { return -1; }

	/**
	 * sets the Sudoku at (x, y) to value
	 * @param x the x coordinate
	 * @param y the y coordinate
	 * @param value the new value
	 */
	static set(x: number, y: number, value: number) { }

	/**
	 * sets the n-th number of the seed
	 * @param n the index
	 * @param seed next seed number
	 */
	static set_seed(n: number, seed: number) { }

	/**
	 * returns the n-th number of the seed
	 * @param n the index
	 */
	static get_seed(n: number) { return -1; }

	/**
	 * initiallises and generates the Sudoku
	 */
	static init() { }

	/**
	 * checks, if value can be placed at (x, y)
	 * @param x the x coordinate
	 * @param y the y coordinate
	 * @param value the value
	 */
	static is_possible(x: number, y: number, value: number) { return false; }

	/**
	 * counts, how many cells are still empty
	 */
	static count_remaining() { return -1; }

	/**
	 * @param difficulty
	 */
	static reduce(difficulty: number) { }
	
	/**
	 * solves the Sudoku
	 */
	static solve() { return false; }
}
