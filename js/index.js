const WebSocket = require('ws');
const readline = require('readline');
const fs = require('fs').promises;
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});
// import { identify } from './user';


async function main() {
  const user_info = await new Promise((resolve) => {
    rl.question('输入用户名与密码：', (input) => {
      resolve(input);
    });
  });
  let socket = new WebSocket("ws://127.0.0.1:8080");
  socket.onopen = function(e) {
    socket.send(user_info);
  };

  socket.onmessage = function(event) {
    if (typeof event.data === 'string') {
      console.log('Received text data:', event.data);
    } else {
      console.log('Received binary data:', event.data);
    }
  };

  socket.onclose = function(event) {
    if (event.wasClean) {
      console.log(`[close] Connection closed cleanly, code=${event.code}, reason=${event.reason}`);
    } else {
      console.log('[close] Connection died');
    }
  };

  socket.onerror = function(error) {
    console.log(`[error] ${error}`);
  };

  while (true) {
    const input = await new Promise((resolve) => {
      rl.question('输入操作(send/close)：', (input) => {
        resolve(input);
      });
    });

    if (input === 'send') {
      await send_msg(socket);
    } else if (input === 'close') {
      socket.send("我关闭了连接");
      socket.close();
      break; // 退出循环
    }
  }
  //rl.close();
}

async function send_msg(socket) {
  const kind = await new Promise((resolve) => {
    rl.question("输入消息类型(text/bin)：", (kind) => {
      resolve(kind);
    });
  });

  if (kind === 'text') {
    const msg = await new Promise((resolve) => {
      rl.question("输入消息：", (msg) => {
        resolve(msg);
      });
    });
    socket.send(msg);
  } else if (kind === 'bin') {
    const msg = await new Promise((resolve) => {
      rl.question("输入消息：", (msg) => {
        resolve(msg);
      });
    });
    socket.send(Buffer.from(msg));
    // const file = await new Promise((resolve) => {
    //   rl.question("输入路径：", (file) => {
    //     resolve(file);
    //   });
    // });
    // try {
    //   const data = await fs.readFile(file);
    //   socket.send(data);
    // } catch (err) {
    //   console.error(err);
    // }
  }
}

main();
