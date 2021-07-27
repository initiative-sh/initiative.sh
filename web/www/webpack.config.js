const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  /*
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
    ],
  },
  */
  plugins: [
    new HtmlWebpackPlugin({
      template: "src/index.html",
    }),
    new CopyWebpackPlugin({
      patterns: [
        { from: "public/*.css" },
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  devServer: {
    contentBase: path.join(__dirname, "public"),
  },
};
