//use std::collections::HashMap;
//use futures_channel::mpsc::{unbounded, UnboundedSender};
//use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
//use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use crate::user::{User, USERS_INFO};
use futures_util::SinkExt;
use futures_util::{lock::Mutex, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
pub struct Server {
    addr: String,
    users: Arc<Mutex<Vec<User>>>,
}

impl Server {
    pub fn bind(address: String) -> Self {
        Server {
            addr: address,
            users: Arc::new(Mutex::new(vec![])),
        }
    }
    pub async fn run(&self) {
        let listener = TcpListener::bind(&self.addr).await.unwrap();
        println!("正在监听 {}", &self.addr);
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            println!("收到一个连接");
            let _users = self.users.clone();
            tokio::task::spawn(Self::client_handler(stream, _users));
        }
    }
    async fn client_handler(stream: TcpStream, users: Arc<Mutex<Vec<User>>>) {
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");

        let (write, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write));
        // 规定建立连接后的第一条消息必须是用户认定消息
        // 格式为用户名  空格 密码
        let mut user_identify_ok: bool = false;
        let mut current_user = String::new();
        // 处理接收到的消息
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => {
                    if user_identify_ok {
                        // 通过验证 正常处理消息
                        //if msg.is_binary() {
                        //    println!("接收到二进制消息");
                        //    Self::broadcast_msg(&mut write, msg, users.clone(), &current_user)
                        //        .await;
                        //} else if msg.is_text() {
                        //    let tmp = msg.to_text().unwrap();
                        //    println!("接收到文本消息: {}", tmp);
                        //}
                        if msg.is_binary() {
                            println!("接收到二进制消息");
                            Self::broadcast_msg(users.clone(), &current_user, msg).await;
                        } else if msg.is_text() {
                            let tmp = msg.to_text().unwrap();
                            println!("接收到文本消息: {}", tmp);
                        }
                    } else {
                        // 未经验证，进行验证
                        let identify_result = Self::user_identify(&msg);
                        if identify_result.0 {
                            user_identify_ok = true;
                            current_user = identify_result.1.to_string();
                            let new_user = User::new(&current_user, write.clone());
                            // 将当前用户加入用户列表
                            println!("新加用户: {:#?}", &new_user);
                            (users.lock().await).push(new_user);
                        } else {
                            // 验证失败，发送中断连接消息
                            let write_arc = write.clone();
                            let mut ws_stream = write_arc.lock().await;
                            if let Err(e) = (*ws_stream).send(Message::Close(None)).await {
                                println!("Error sending message: {}", e);
                            }
                            // 无论如何，都要跳出循环，中断连接
                            break;
                        }
                    }
                    //println!("Received a message: {}", msg.to_text().unwrap());
                    //if let Err(e) = write.send(msg).await {
                    //    println!("Error sending message: {}", e);
                    //    break;
                    //}
                }
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    }
    fn user_identify(msg: &Message) -> (bool, &str) {
        if !msg.is_text() {
            return (false, "");
        }
        let user_info = msg.to_text().unwrap();
        let info: Vec<&str> = user_info.splitn(2, " ").collect();
        if let Some(&name) = info.get(0) {
            let passwd_maybe = USERS_INFO.get(name);
            match passwd_maybe {
                Some(passwd) => {
                    if let Some(passwd_about_to_identify) = info.get(1) {
                        if passwd_about_to_identify == passwd {
                            // 验证通过
                            return (true, name);
                        } else {
                            // 密码错误
                            return (false, "");
                        }
                    } else {
                        // 传递的信息不完整
                        return (false, "");
                    }
                }
                None => {
                    // 没有这个用户
                    return (false, "");
                }
            }
        } else {
            // 传递的信息不完整
            return (false, "");
        }
    }
    async fn broadcast_msg(users: Arc<Mutex<Vec<User>>>, current_user: &str, msg: Message) {
        let mut users = users.lock().await;
        for u in &mut (*users) {
            let u_id = u.get_user_id();
            if u_id != current_user {
                println!("发送一条广播消息给 {}", u_id);
                let msg_copy = msg.clone();
                u.send_msg(msg_copy).await;
            }
        }
    }
    //async fn broadcast_msg(
    //    ws_stream: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    //    msg: Message,
    //    users: Arc<Mutex<Vec<User>>>,
    //    current_user: &str,
    //) {
    //    let users = users.lock().await;
    //    for u in &(*users) {
    //        let u_id = u.get_user_id();
    //        if u_id != " " {
    //            //if u_id != current_user {
    //            // 发送数据
    //            let msg_copy = msg.clone();
    //            if let Err(e) = ws_stream.send(msg_copy).await {
    //                println!("Error sending message: {}", e)
    //            } else {
    //                println!("广播一个二进制消息");
    //            }
    //        }
    //    }
    //}
    //fn get_user_id(msg: &Message) -> String {
    //    let info = msg.to_text().unwrap();
    //    info.splitn(2, " ").collect();
    //}
}
