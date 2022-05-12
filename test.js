(function (){
    const WebSocket = require("ws");


    const ws = new WebSocket('ws://aicp-test');
    
    ws.on('open', function open() {
      console.log("连接成功")
      setInterval(() => {
        ws.send(new Date().toString());
      }, 2000);
    });
    
    ws.on('message', function message(data) {
      console.log('received: %s', data);
    });


    ws.on('error', function message(e) {
        console.log('error', e);
      });
}())





(function (){
    const WebSocket = require("ws");


    const ws = new WebSocket('ws://127.0.0.1:3333');
    
    ws.on('open', function open() {
      console.log("连接成功")

    });
    
    ws.on('message', function message(data) {
      console.log('received: %s', data);
    });


    ws.on('error', function message(e) {
        console.log('error', e);
      });
}())








(function (){
    var socket = new WebSocket("ws://127.0.0.1:3333");
    console.log(socket.readyState) //0 - 连接尚未建立

    //2.连接打开时触发
    socket.onopen = function (event) {
      console.log("1 - 连接已建立，可以进行通信", socket.readyState) //1 - 连接已建立，可以进行通信

    //   // 向服务器发送数据的方法（将要发送的数据放入队列）
    //   socket.send("Hello WebSockets!");
    //   console.log('队列中等待传输的 UTF-8 文本字节数', socket.bufferedAmount) //队列中等待传输的 UTF-8 文本字节数。
    };

    //3.客户端接收服务端数据时触发
    socket.onmessage = function (event) {
      // 接收服务器返回的数据
      console.log("Received Message: " + event.data);

   
    };

    //4.通信发生错误时触发
    socket.onerror = function (event) {
      console.log("连接错误", event, socket.readyState) //3 - 连接已经关闭
    };

    //5.连接关闭时触发
    socket.onclose = function (event) {
      console.log("连接关闭",socket.readyState) //3 - 连接已经关闭
    };


}())

///////////////////////////////////////////////////////////////////////////////

(function (){
    var socket = new WebSocket("ws://aicp-test-ws.stage.dm-ai.com:5555");
    console.log(socket.readyState) //0 - 连接尚未建立

    //2.连接打开时触发
    socket.onopen = function (event) {
      console.log("1 - 连接已建立，可以进行通信", socket.readyState) //1 - 连接已建立，可以进行通信

      // 向服务器发送数据的方法（将要发送的数据放入队列）
      setInterval(() => {
        socket.send(new Date().toString());
      }, 2000);
      
    //   console.log('队列中等待传输的 UTF-8 文本字节数', socket.bufferedAmount) //队列中等待传输的 UTF-8 文本字节数。
    };

    //3.客户端接收服务端数据时触发
    socket.onmessage = function (event) {
      // 接收服务器返回的数据
      console.log("Received Message: " + event.data);

   
    };

    //4.通信发生错误时触发
    socket.onerror = function (event) {
      console.log("连接错误", event, socket.readyState) //3 - 连接已经关闭
    };

    //5.连接关闭时触发
    socket.onclose = function (event) {
      console.log("连接关闭",socket.readyState) //3 - 连接已经关闭
    };


}())











