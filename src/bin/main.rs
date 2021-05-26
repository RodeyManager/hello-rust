use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use hello_rust::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    start();
}

// 开始之旅
fn start() {
    let server = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // take 方法定义于 Iterator trait，这里限制循环最多头 5 次
    for stream in server.incoming().take(5) {
        pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }

    println!("Shutting down.");
}

// 处理请求
fn handle_connection(mut stream: TcpStream) {
    let mut request = [0; 1024];

    stream.read(&mut request).unwrap();
    // println!("Request: \n{}\n", String::from_utf8_lossy(&buffer[..]));

    send_response(stream, request);
}

// 发送数据
fn send_response(mut stream: TcpStream, req: [u8; 1024]) {
    // 验证请求并有选择的进行响应
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // 本人开始想法
    // let mut status = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    // let mut filename = "404";

    // if req.starts_with(get) {
    //   status = "HTTP/1.1 200 OK\r\n\r\n";
    //   filename = "index";
    // }

    // 书上的做法（👍）
    let (status, filename) = if req.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index")
    } else if req.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404")
    };

    let html_content = read_html(filename);
    let response = format!("{}\r\n{}", status, html_content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// 读取对应 html 模板内容
fn read_html(name: &str) -> String {
    // let html = fs::read_to_string("./views/" + name + ".html");
    let html = fs::read_to_string(format!("./views/{}.html", name));
    match html {
        Ok(content) => content,
        Err(_) => return String::from("404 Not fount page"),
    }
}
