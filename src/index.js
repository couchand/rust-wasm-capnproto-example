const { extractSlice, getSliceData, newSlice } = require("./io");
const { loadMessage, writeMessage } = require("./message");

fetch("capnproto_example.wasm")
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(results => {
    const {
      memory,
      make_point,
      destroy_point,
      x,
      y,
      alloc,
      dealloc,
    } = results.instance.exports;

    const pointer = make_point(42, Math.PI);
    const [slice, size] = extractSlice(memory, pointer);
    const pointMessage = getSliceData(memory, slice, size);
    const point = loadMessage(pointMessage);

    document.querySelector("#wasm-js-x").innerHTML = point.getX();
    document.querySelector("#wasm-js-y").innerHTML = point.getY();

    document.querySelector("#wasm-wasm-x").innerHTML = x(pointer);
    document.querySelector("#wasm-wasm-y").innerHTML = y(pointer);

    destroy_point(pointer);

    const pointBack = writeMessage(42, Math.PI);
    const pointerBack = newSlice(memory, alloc, pointBack);

    document.querySelector("#js-wasm-x").innerHTML = x(pointerBack);
    document.querySelector("#js-wasm-y").innerHTML = y(pointerBack);
  });
