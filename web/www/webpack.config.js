const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].[contenthash].js",
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
      templateParameters: (compilation, files, tags, options) => {
        compilation.getAssets()
          .map(asset => asset.name)
          .filter(name => name.endsWith(".css"))
          .forEach(name => files.css.push(name));

        return {
          htmlWebpackPlugin: {
            tags: tags,
            files: files,
            options: options,
          },
        };
      },
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, "node_modules/source-code-pro"),
          to: path.resolve(__dirname, "dist/source-code-pro"),
        },
        {
          from: path.resolve(__dirname, "public/*.css"),
          to: path.resolve(__dirname, "dist/[name].[contenthash][ext]"),
        },
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
