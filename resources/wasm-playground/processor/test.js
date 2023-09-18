const fs = require('fs');

const wasmBuffer = fs.readFileSync(__dirname + '/processor.wasm');
WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
    // Exported function live under instance.exports
    // const sum = add(5, 6);
    // console.log(sum); // Outputs: 11

    console.log(wasmModule.instance.exports);
    const { process } = wasmModule.instance.exports;

    for (let i = 0; i < 48000; i++) {
        console.log(process());
    }
});