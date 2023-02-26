const wasm = require('./pkg/canvaskit')
const options = JSON.stringify({
  size: [1200, 800],
  background: { type: 'Rgba', value: [255, 255, 0, 200] },
  "graphics":[
    { "type": "Rectangle", "value": { "corner": [24, 24, 24, 24], "color": { "type": "Rgba", "value": [255, 0, 0, 120] }, "position": [30, 30], "size": [400, 300] } },
    { "type": "Rectangle", "value": { "corner": [24, 24, 24, 24], "color": { "type": "Rgba", "value": [255, 0, 255, 100] }, "position": [100, 50], "size": [700, 600] } },
  ]
})

const buff = wasm.draw(options)

require('fs').writeFileSync('test.png', buff, { flag: 'w+' })