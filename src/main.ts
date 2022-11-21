
import * as store from "store";

let Wasm: typeof import("../pkg/index.js");

let data: {
    /**
     * the difficulty level of the next Sudoku.
     * 0 => already solved (~4ms to generate)
     * 5 => most difficult (~12ms to generate)
     */
    difficulty: number,
    /**
     * the random seed used to generate the Sudoku.
     * needs to be 32 bytes aka 8x u32
     */
    seed: number[],
    /**
     * an array containing all placed numbers
     */
    placed: number[][]
} = {
    difficulty: 1,
    seed: [0, 0, 0, 0, 0, 0, 0, 0],
    placed: []
};

function save() {
    store.set("sudokuData", data);
}

let wasmPromise = import("../pkg/index.js");
let loadedPromise = new Promise(window.addEventListener.bind(window, "load"));

Promise.all([wasmPromise, loadedPromise]).then(([wasm]) => {
    Wasm = wasm;
    window["wasm"] = wasm;
    document.getElementById("gen").addEventListener("click", () => gen());
    const slider = document.getElementById("difficulty") as HTMLInputElement;
    slider.addEventListener("input", difficultyChange);

    const stored = store.get("sudokuData");
    if (stored) {
        data = stored;
        slider.value = data.difficulty.toString();
        gen(true);
    }
});

function genSeed() {
    for (let i = 0; i < 8; i++) {
        data.seed[i] = Math.floor(Math.random() * 0xffffffff);
    }
}

/**
 * generates a new Sudoku and sets up the tables
 */
function gen(loaded: boolean = false) {
    if (!loaded) {
        genSeed();
        data.placed = [];
    }

    while (true) {
        Wasm.init(new Uint32Array(data.seed));
        if (Wasm.count_remaining() > 0) {
            genSeed();
        } else {
            break;
        }
    }
    save();

    Wasm.reduce(data.difficulty);

    let table = document.getElementById("table") as HTMLTableElement;
    if (!table) {
        table = createTable();
    }

    for (let y = 0; y < 9; y++) {
        for (let x = 0; x < 9; x++) {
            const cell = table.rows[y].cells[x];
            const content = Wasm.get(x, y);
            cell.innerText = content !== 0 ? content.toString() : "";
        }
    }

    Wasm.solve();

    if (loaded && data.placed.length > 0) {
        let index = 0;
        let handle = setInterval(() => {
            console.log("placing", data.placed[index]);
            let [x, y] = data.placed[index];
            const cell = table.rows[y].cells[x];
            cell.innerText = Wasm.get(x, y).toString();

            if (++index >= data.placed.length) {
                clearInterval(handle);
                enableEdits();
            }
        }, 100);
    } else {
        enableEdits();
    }
}

/**
 * creates a html table from the wasm sudoku
 */
function createTable() {
    const table = document.createElement("table");
    table.id = "table";
    for (let y = 0; y < 9; y++) {
        const tr = document.createElement("tr");
        table.appendChild(tr);

        for (let x = 0; x < 9; x++) {
            const cell = document.createElement("td");
            tr.appendChild(cell);
            if (x % 3 == 0) {
                cell.style.borderLeftWidth = "2px";
            }
            if (y % 3 == 0) {
                cell.style.borderTopWidth = "2px";
            }
            cell.addEventListener("input", event => {
                onInput(cell, x, y);
                event.preventDefault();
            });
        }
    }
    document.body.appendChild(table);
    return table;
}

function enableEdits() {
    const table = document.getElementById("table") as HTMLTableElement;
    for (let y = 0; y < 9; y++) {
        for (let x = 0; x < 9; x++) {
            const cell = table.rows[y].cells[x];
            cell.contentEditable = (cell.innerText === "").toString();
        }
    }
}

function onInput(element: HTMLTableCellElement, x: number, y: number) {
    const solution = Wasm.get(x, y);
    if (element.innerText === solution.toString()) {
        if (Wasm.count_remaining() === 1) {
            alert("Congratulations!");
        }
        element.contentEditable = "false";

        data.placed.push([x, y]);
        save();
    }
}

/**
 * changes the difficulty level
 * @param event the onInput event
 */
function difficultyChange(this: HTMLInputElement, event: Event) {
    data.difficulty = parseInt(this.value);
}
