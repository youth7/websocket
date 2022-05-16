(function () {
  const WebSocket = require("ws");


  const ws = new WebSocket('ws://127.0.0.1:3333');
  ws.on('open', function open() {
    console.log("连接成功")
    ws.send("a".repeat(125));
    ws.send("b".repeat(0xFFF));
    ws.send("c".repeat(0xFFFF * 2));
    ws.send("11", {fin:false});
    ws.send("22", {fin:false});
    ws.send("33", {fin:false});
    ws.send("44", {fin:true});
    // ws.close();
  });

  ws.on('message', function message(data) {
    console.log('received: %s', data);
  });


  ws.on('error', function message(e) {
    console.log('error', e);
  });

  ws.on("close", function (code, reason) {
    console.log("关闭连接", code, reason.toString());

  })
}())