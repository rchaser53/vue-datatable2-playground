const path = require('path')
const fetch = require('node-fetch')
const express = require('express')

const fakeData = require('./fakeData')
const app = express()
const server = require('http').createServer(app)

app.get('/', async (req, res) => {
  try {
    const pre = await fetch('https://api.github.com/repos/rchaser53/vue-table-playground/commits')
    const ret = await pre.json()
    res.send(JSON.stringify(ret))
  }
  catch (err) {
    console.error(err)
  }
});

app.get('/tabledata', async (req, res) => {
  try {
    res.header('Access-Control-Allow-Origin', '*')
    res.send(JSON.stringify(fakeData))
  }
  catch (err) {
    console.error(err)
  }
});

app.get('/index', (req, res) => {
  res.sendFile(path.join(__dirname, 'index.html'))
})

server.listen(3100, () => {
  console.log('run server');
});