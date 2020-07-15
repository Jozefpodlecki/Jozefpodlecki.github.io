const { VueLoaderPlugin } = require('vue-loader');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const TsconfigPathsPlugin = require('tsconfig-paths-webpack-plugin');
const HtmlPlugin = require('html-webpack-plugin');
const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');
 
const outputPath = __dirname + '/dist'

module.exports = {
  mode: "development",
  stats: 'errors-only',
  entry: './src/index.ts',
  output: {
    filename: '[name].js',
    publicPath: '/',
    path: outputPath
  },
  devServer: {
    contentBase: path.join(__dirname, 'public'),
    compress: true,
    port: 9000,
    disableHostCheck: true
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.vue'],
    plugins: [
      new TsconfigPathsPlugin({ /*configFile: "./tsconfig.json" */ }),
    ]
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        use: {
          loader: 'vue-loader',
        },
      },
      {
        test: /\.(s)css/,
        use: [
          "style-loader",
          "css-loader",
          "sass-loader"
        ]
      },
      {
        test: /\.ts$/,
        use: [
          {
            loader: 'ts-loader',
            options: {
              appendTsSuffixTo: [/\.vue$/],
            }
          }
        ]
      }
    ],
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new VueLoaderPlugin(),
    new HtmlPlugin({
      title: 'Józef Podlecki',
      template: 'index.html.ejs',
      chunksSortMode: 'dependency' 
    }),
    new CopyPlugin([{
      from: 'src/assets',
      to: 'assets' 
    }])
  ],
};