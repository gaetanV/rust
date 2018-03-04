const webpack = require('webpack');
const path = require('path');

module.exports = {
	entry: __dirname +'/src/index.js',
	output: {
    filename: 'boot.js',
    path: __dirname + '/build',
	},
	module: {
	  rules: [
		{
		  test: /\.rs/,
		  use: [
				{
					loader: 'wasm-loader'
				}, 
				{
					loader: 'rust-native-wasm-loader',
					options: { release: true },
				}
			]
		}
	  ]
	}
  }