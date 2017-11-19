const path = require('path')
const fs = require('fs')
const casual = require('casual')

casual.define('user', function () {
  return {
    email: casual.email,
    firstname: casual.first_name,
    lastname: casual.last_name,
    password: casual.password
  }
})

let tempArray = []
for (let i = 0; i < 100; i++) {
  tempArray.push(JSON.stringify(casual.user))
}

const writeData = `
module.exports = {
  data: [${tempArray}]
}
`

fs.writeFileSync(path.join(__dirname, 'fakeData.js'), writeData)