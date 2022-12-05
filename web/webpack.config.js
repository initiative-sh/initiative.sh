const HtmlWebpackPlugin = require('html-webpack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

const templateParameters = (compilation, files, tags, options) => {
  compilation.getAssets()
    .map(asset => asset.name)
    .filter(name => name.endsWith('.css'))
    .forEach(name => files.css.push(name))

  return {
    htmlWebpackPlugin: {
      tags: tags,
      files: files,
      options: options,
    },
  }
}

module.exports = {
  entry: './js/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].[contenthash].js',
  },
  mode: 'development',
  /*
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader'],
      },
    ],
  },
    */
  plugins: [
    new HtmlWebpackPlugin({
      filename: 'index.html',
      template: 'js/index.html',
      templateParameters: templateParameters,
    }),
    new HtmlWebpackPlugin({
      filename: 'privacy.html',
      inject: false,
      template: 'js/privacy.html',
      templateParameters: templateParameters,
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, 'node_modules/source-code-pro'),
          to: path.resolve(__dirname, 'dist/source-code-pro'),
        },
        {
          from: path.resolve(__dirname, 'static/*.css'),
          to: path.resolve(__dirname, 'dist/[name].[contenthash][ext]'),
        },
        {
          from: path.resolve(__dirname, 'static/favicon.ico'),
          to: path.resolve(__dirname, 'dist/favicon.ico'),
        },
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  /*
  devServer: {
    contentBase: path.join(__dirname, 'static'),
  },
  */
  resolve: {
    alias: {
      'initiative-web': path.resolve(__dirname, 'pkg/initiative_web.js'),
    },
  },
}
