const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const ExtractTextPlugin = require('extract-text-webpack-plugin');

const entryFile = "./Application/index.tsx"
const indexHtmlPath = "./index.html"
const outputPath = path.resolve(__dirname, 'build');
const outputFile = "bundle.min.js";
const contentBase = path.join(__dirname, './build');
const htmlTemplate = path.resolve('.', 'Application', 'index.html');

module.exports = {
    mode: "development",
    devtool: "inline-source-map",
    entry: entryFile,
    output: {
        path: outputPath,
        filename: outputFile,
        publicPath: '/'
    },
    devServer: {
      contentBase: contentBase,
      compress: true,
      port: 9000,
      historyApiFallback: true
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".md", ".css"]
    },
    module: {
        rules: [
        { 
          test: /\.tsx?$/,
          loader: "ts-loader" 
        },
        {
          test: /\.md$/,
          use: [
          {
              loader: "html-loader"
          },
          {
              loader: "markdown-loader",
              options: {
              }
          }
          ]
        },
        {
          test: /\.(sa|sc|c)ss$/,
          use: ExtractTextPlugin.extract({
            publicPath: "../",
            fallback:'style-loader',
            use:['css-loader','sass-loader'],
        })
        }
      ]
    },
    plugins: [
        new ExtractTextPlugin({filename:'app.bundle.css'}),
        new HtmlWebpackPlugin({
          template: htmlTemplate,
          filename: indexHtmlPath
        }),
        new CopyWebpackPlugin([
          { from: 'Application/Posts' },
          { from: 'Application/Data' }
        ])
      ]
  };