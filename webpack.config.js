const path = require("path");

module.exports = {
  mode: "development",
  entry: "./js/index",
  output: {
    path: path.resolve(__dirname, "site"),
    filename: "bundle.js",
  },
  devtool: "source-map",
  target: "web",
};
