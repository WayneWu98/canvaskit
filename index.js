import init, * as canvas from './pkg/canvaskit.js'

function arrayBufferToBase64(buffer) {
  var binary = '';
  var bytes = new Uint8Array(buffer);
  var len = bytes.byteLength;
  for (var i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}

init('./pkg/canvaskit_bg.wasm')
  .then(() => fetch('public/xiaowei.ttf'))
  .then(res => res.arrayBuffer())
  .then(xiaowei => {
    console.log('init finished')
    console.time('draw')
    const r = 20
    const options = JSON.stringify({
      size: [1000, 1000],
      background: { type: 'Rgba', value: [255, 255, 0, 100] },
      fontSet: { xiaowei: Array.from(new Uint8Array(xiaowei)) },
      "graphics": [
        // { "type": "Rectangle", "value": { "color": { "type": "Rgba", "value": [255, 0, 0, 120] }, "position": [30, 30], "size": [400, 400] } },
        // { "type": "Line", "value": { "color": [255, 0, 0, 120], "from": { x: 0, y: 0 }, "to": { x: 500, y: 100 }, width: 10, shadow: { x: 0, y: 10, blur: 10, color: [0, 255, 0, 200] } } },
        { "type": "Rectangle", 
        "value": { 
          "corner": new Array(4).fill(r), 
          "color": { 
            "type": "Gradient", 
            "value": { 
              angle: 225, 
              stops: [{ position: { type: 'Percent', value: 0 }, color: [245, 224, 20, 255] }, { position: { type: 'Percent', value: 1 }, color: [190, 122, 240, 255] }]
            }
          },
          shadow: { x: 0, y: 5, blur: 20, spread: 5, color: [0, 255, 0, 255] },
          "position": { x: 80, y: 200 },
          // "size": { width: 800, height: 200 },
          padding: [24, 24, 24, 24],
          children: [
            {
              type: "Rectangle",
              value: {
                // position: { x: 0, y: 0 },
                size: { width: 200, height: 200 },
                color: {
                  type: "Rgba",
                  value: [255, 0, 255, 255]
                }
              }
            },
            {
              type: "Rectangle",
              value: {
                position: { x: 0 },
                size: { width: 200, height: 200 },
                color: {
                  type: "Rgba",
                  value: [255, 0, 0, 255]
                }
              }
            },
          ]
        } },

      ]
    })
    // const buff = canvas.draw(options)
    const buff = canvas.draw(options)
    console.timeEnd("draw")
    console.time('parse')
    const url = 'data:image/png;base64,' + arrayBufferToBase64(buff)
    console.timeEnd("parse")
    document.querySelector('#img').src = url
  })