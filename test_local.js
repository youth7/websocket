
(function () {
  const WebSocket = require("ws");


  const ws = new WebSocket('ws://127.0.0.1:3333');
  ws.on('open', function open() {
    let i = 0;
    let flag = setInterval(() => {
      ws.send(Math.random().toString() + "1".repeat(200));
      if (i++ > 0) {
        ws.close();
        clearInterval(flag);
      }
    }, 2000);
    console.log("连接成功")

  });

  ws.on('message', function message(data) {
    console.log('received: %s', data);
  });


  ws.on('error', function message(e) {
    console.log('error', e);
  });

  ws.on("close",function(code, reason){
    console.log("关闭连接", code, reason.toString());
    
  })
}())