const wasm = import("../target/bindgen/sudoku" /* webpackChunkName:"sudoku" */);

wasm.then(module => {
	const rootEl = document.getElementById('app');
	module.start(rootEl);
});
