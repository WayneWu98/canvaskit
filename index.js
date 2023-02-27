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
  .then(() => {
    console.log('init finished')
    console.time('draw')
    const r = 9999
    const options = JSON.stringify({
      size: [1000, 1000],
      background: { type: 'Rgba', value: [255, 255, 0, 100] },
      "graphics": [
        // { "type": "Rectangle", "value": { "color": { "type": "Rgba", "value": [255, 0, 0, 120] }, "position": [30, 30], "size": [400, 400] } },
        { "type": "Line", "value": { "color": [255, 0, 0, 120], "from": [30, 30], "to": [400, 400], width: 10, shadow: { x: 220, y: 10, blur: 10, color: [0, 255, 0, 200] } } },
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
          "position": [80, 200], "size": [600, 600],
          // shadow: {
          //   x: 0,
          //   y: 0,
          //   spread: 20,
          //   blur: 10,
          //   color: [255, 0, 0, 100],
          // }
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