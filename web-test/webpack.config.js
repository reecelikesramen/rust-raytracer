const path = require("path")

module.exports = {
  target: "web",
  resolve: {
    extensions: [".js", ".mjs"], // Ensure `.mjs` is included if necessary
    modules: [path.resolve(__dirname, "node_modules")],
  },
  devtool: "source-map",
}
