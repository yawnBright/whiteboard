use futures_util::SinkExt;
//use futures_util::SinkExt;
use futures_util::{lock::Mutex, stream::SplitSink};
use lazy_static::lazy_static;
use std::sync::Arc;
//use std::sync::Arc;
use std::{collections::HashMap, io::BufRead};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

lazy_static! {
    #[derive(Debug)]
    pub static ref USERS_INFO: Arc::<HashMap<String, String>> = Arc::new(get_all_users_info());
}

fn get_all_users_info() -> HashMap<String, String> {
    let mut users: HashMap<String, String> = HashMap::new();
    let fs = std::fs::File::open("./resource/users.txt").unwrap();
    let reader = std::io::BufReader::new(fs);
    // 按行迭代文件内容
    for line in reader.lines() {
        let info = line.unwrap();
        let tmp: Vec<&str> = info.splitn(2, " ").collect();
        if let Some(&name) = tmp.get(0) {
            if let Some(&passwd) = tmp.get(1) {
                users.insert(name.to_string(), passwd.to_string());
                print!("name: {}, passwd: {}\n", name, passwd);
            }
        }
        // 处理每行，这里只是打印出来
    }
    users
}
#[derive(Debug)]
pub struct User {
    name: String,
    ws_stream: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}

impl User {
    pub fn new(
        name: &str,
        ws_stream: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Self {
        User {
            name: name.to_string(),
            ws_stream,
        }
    }
    pub fn get_user_id(&self) -> &str {
        &self.name
    }
    pub async fn send_msg(&mut self, msg: Message) {
        let mut stream = self.ws_stream.lock().await;
        if let Err(e) = (*stream).send(msg).await {
            println!("Send Message err: {} {}-{}", e, file!(), line!());
        }
    }
}
