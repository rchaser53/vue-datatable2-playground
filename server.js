const fetch = require('node-fetch')
const express = require('express')

const app = express()
const server = require('http').createServer(app)

app.get('*', async (req, res) => {
  try {
    const pre = await fetch('https://api.github.com/repos/rchaser53/vue-table-playground/commits')
    const ret = await pre.json()
    res.send(JSON.stringify(ret))
  }
  catch (err) {
    console.error(err)
  }
});

server.listen(3000, () => {
  console.log('run server');
});