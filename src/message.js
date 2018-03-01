const capnp = require("capnp-ts");

const Point = require("../build/schema/example.capnp").Point;

function loadMessage(buffer) {
  const message = new capnp.Message(buffer);
  return message.getRoot(Point);
}

function writeMessage(x, y) {
  const message = new capnp.Message();
  const point = message.initRoot(Point);

  point.setX(x);
  point.setY(y);

  return new Uint8Array(message.toPackedArrayBuffer());
}

module.exports = {
  loadMessage,
  writeMessage,
};
