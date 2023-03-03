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
      fontSet: { xiaowei: Array.from(new Uint8Array(xiaowei)) },
      graphic: { 
        // "corner": new Array(4).fill(r), 
        "color": { 
          "type": "Rgba", 
          "value": [0, 0, 0, 0]
        },
        // shadow: { x: 0, y: 0, blur: 10, spread: 20, color: [0, 255, 0, 255] },
        "position": { x: 0, y: 0 },
        "size": { width: 1000 },
        color: {
          type: "Rgba",
          value: [244, 24, 12, 50],
        },
        clip: true,
        padding: [24, 24, 24, 24],
        children: [
          {
            type: "Container",
            value: {
              position: { x: 200, y: 0 },
              // size: {  height: 500 },
              // color: {
              //   type: "Rgba",
              //   value: [255, 0, 255, 255]
              // },
              corner: [20, 20, 20, 20],
              // shadow: { x: 0, y: 0, blur: 10, spread: 20, color: [0, 255, 0, 255] },
              align: 'Center',
              clip: true,
              padding: [32, 32, 32, 32],
              children: [
                {
                  type: "Container",
                  value: {
                    position: { x: 0, y: 0 },
                    size: { width: 1000, height: 600 },
                    color: {
                      type: "Rgba",
                      value: [0, 0, 255, 80]
                    },
                    // shadow: { x: 0, y: 0, blur: 10, spread: 20, color: [0, 255, 0, 255] },
                    // align: 'Right'
                    children: [
                      
                    ]
                  }
                },
                {
                  type: 'Text',
                  value: {
                    content: 'hello, world!',
                    color: [255, 0, 0, 255],
                    size: 32,
                    lineHeight: 40,
                    position: {
                      x: 0,
                      y: 0,
                    }
                  }
                }
              ]
            }
          },
          // {
          //   type: 'Container',
          //   value: {
          //     position: { x: 0 },
          //     size: { height: 20 },color: {
          //       type: "Rgba",
          //       value: [255, 0, 255, 0]
          //     },
          //   }
          // },
          {
            type: "Line",
            value: {
              width: 20,
              color: [0, 0, 0, 255],
              shadow: { x: 80, y: 10, blur: 10, color: [255, 0, 0, 255] },
              from: { x: 20, y: 20 },
              to: { x: 500, y: 80 },
            }
          },
          // {
          //   type: 'Container',
          //   value: {
          //     position: { x: 0 },
          //     size: { height: 20 },color: {
          //       type: "Rgba",
          //       value: [255, 255, 255, 150]
          //     },
          //   }
          // },
          // {
          //   type: "Rectangle",
          //   value: {
          //     position: { x: 0 },
          //     size: { width: 200, height: 200 },
          //     color: {
          //       type: "Rgba",
          //       value: [255, 0, 0, 255]
          //     }
          //   }
          // },
        ]
      }
    })
    // const buff = canvas.draw(options)
    const buff = canvas.draw(options)
    console.timeEnd("draw")
    console.time('parse')
    const url = 'data:image/png;base64,' + arrayBufferToBase64(buff)
    console.timeEnd("parse")
    document.querySelector('#img').src = url
  })