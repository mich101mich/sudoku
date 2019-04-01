
import Wasm from "./wasm";
import * as store from "store";

let data: {
	/**
	 * the difficulty level of the next Sudoku.
	 * 0 => already solved (~4ms to generate)
	 * 5 => most difficult (~12ms to generate)
	 */
	difficulty: number,
	/**
	 * the random seed used to generate the Sudoku.
	 * needs to be 16 bytes aka 4x i32
	 */
	seed: number[],
	/**
	 * an array containing all placed numbers
	 */
	placed: number[][]
} = {
		difficulty: 1,
		seed: [0, 0, 0, 0],
		placed: []
	};

function save() {
	store.set("sudokuData", data);
}

window.addEventListener("load", (event) => {
	document.getElementById("gen").addEventListener("click", () => gen());
	const slider = document.getElementById("difficulty") as HTMLInputElement;
	slider.addEventListener("input", difficultyChange);

	const stored = store.get("sudokuData");
	if (stored) {
		data = stored;
		slider.value = data.difficulty.toString();
	}

	fetch("lib.wasm")
		.then(file => file.arrayBuffer())
		// @ts-ignore
		.then(buffer => WebAssembly.instantiate(buffer, {}))
		.then(result => {
			const src = result.instance.exports
			for (const key in src) {
				//@ts-ignore
				Wasm[key] = src[key];
			}
			//@ts-ignore
			window["wasm"] = Wasm;

			if (stored) {
				gen(true);
			}
		});
});

function genSeed() {
	for (let i = 0; i < 4; i++) {
		data.seed[i] = Math.floor(Math.random() * 0xffffffff);
	}
	data.seed.forEach((seed, n) => Wasm.set_seed(n, seed));
}

/**
 * generates a new Sudoku and sets up the tables
 */
function gen(loaded: boolean = false) {
	if (!loaded) {
		genSeed();
		data.placed = [];
	} else {
		data.seed.forEach((seed, n) => Wasm.set_seed(n, seed));
	}

	while (true) {
		Wasm.init();
		if (Wasm.count_remaining() > 0) {
			genSeed();
		} else {
			break;
		}
	}
	save();

	let prevTable = document.getElementById("table");
	if (prevTable) {
		prevTable.remove();
	}

	Wasm.reduce(data.difficulty);

	const table = genTable();
	table.id = "table";
	document.body.appendChild(table);

	Wasm.solve();

	if (loaded && data.placed.length > 0) {
		let index = 0;
		let handle = setInterval(() => {

			let [x, y] = data.placed[index];
			const input = document.getElementById(x + "-" + y);
			if (!input) {
				alert("wut?");
				clearInterval(handle);
			}
			input.parentElement.innerText = Wasm.get(x, y).toString();
			input.remove();

			if (++index >= data.placed.length) {
				clearInterval(handle);
			}
		}, 100);
	}
}

/**
 * creates a html table from the wasm sudoku
 */
function genTable() {
	const table = document.createElement("table");
	for (let y = 0; y < 9; y++) {
		const tr = document.createElement("tr");
		table.appendChild(tr);

		for (let x = 0; x < 9; x++) {
			const cell = document.createElement("td");
			tr.appendChild(cell);
			if (x % 3 == 0)
				cell.style.borderLeftWidth = "2px";
			if (y % 3 == 0)
				cell.style.borderTopWidth = "2px";
			const content = Wasm.get(x, y);
			const id = x + "-" + y;
			if (content !== 0) {
				cell.id = id;
				cell.innerText = content.toString();
			} else {
				const input = document.createElement("input");
				cell.appendChild(input);
				input.id = id;
				input.type = "number";
				input.classList.add("input");
				input.addEventListener("input", onInput);
			}
		}
	}
	return table;
}

function onInput(this: HTMLInputElement, event: Event) {
	const [x, y] = this.id.split("-").map(s => parseInt(s));
	const solution = Wasm.get(x, y);
	if (this.value.trim() === solution.toString()) {
		const remain = document.getElementsByClassName("input").length;
		if (remain === 1) {
			alert("Congratulations!");
		}
		this.parentElement.innerText = solution.toString();
		this.remove();

		data.placed.push([x, y]);
		save();
	}
	event.preventDefault();
}

/**
 * changes the difficulty level
 * @param event the onInput event
 */
function difficultyChange(this: HTMLInputElement, event: Event) {
	data.difficulty = parseInt(this.value);
}
