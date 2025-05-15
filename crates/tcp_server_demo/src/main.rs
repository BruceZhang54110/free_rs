use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("无法从客户端读取");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("收到请求：{}", request);
     if request.contains("bye") {
        println!("客户端请求退出，关闭服务器");
        break;
    }
    let respond = "hello client";
    stream
        .write(respond.as_bytes())
        .expect("无法将响应写入客户端");
    stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("无法绑定到地址");
    println!("服务器正在运行，等待连接...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("客户端已连接：{}", stream.peer_addr().unwrap());
                handle_client(stream);
                
                
            }
            Err(e) => {
                println!("连接失败：{}", e);
            }
        }
    }

    println!("服务器关闭");
}
