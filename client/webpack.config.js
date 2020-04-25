const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
  entry: {
    index: './src/index.js'
  },
  output: {
    filename: '[name]-[hash].js',
    path: path.resolve(__dirname, 'dist'),
    publicPath: '/'
  },
  devtool: 'cheap-source-map',
  module: {
    rules: [
      {
        test: /\.(png|jpg|gif|eot|woff2?|ttf|svg|ico)$/,
        loader: 'file-loader'
      },
      {
        test: /\.s?css$/,
        use: [
          "style-loader",       // creates style nodes from JS strings
          "css-loader",         // translates CSS into CommonJS
          "sass-loader" // compiles Sass to CSS, using Node Sass by default
        ]
      }
    ]
  },
  resolve: {
    extensions: [ '.js', '.sass', '.scss', 'css', '.rs', '.png' ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      favicon: './assets/favicon.ico',
      template: 'src/index.html'
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
      watchDirectories: [
        path.resolve(__dirname, "../common")
      ]
    }),
    new webpack.HotModuleReplacementPlugin()
  ],
  devServer: {
    host: '0.0.0.0',
    port: 7777,
    contentBase: path.resolve(__dirname, 'dist'),
    proxy: {
      '/api': 'http://server:7778',
    },
    historyApiFallback: {
      rewrites: [
        { from: /./, to: '/' }
      ]
    },
    hot: true
  }
};
