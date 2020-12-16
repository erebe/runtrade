const Dotenv = require('dotenv-webpack');

module.exports = {
  configureWebpack: {
    plugins: [
        (process.env.NODE_ENV === "production")
            ? new Dotenv({safe: true})
            : new Dotenv({path: '.env.dev', safe: true})
    ]
  }
}

