use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // 尝试连接到服务器
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("成功连接到服务器！");

            // 发送数据给服务器
            let message = "Hello from client!";
            stream.write(message.as_bytes())?;
            println!("已发送消息：{}", message.trim());

            // 接收服务器的响应
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer)?;
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("收到服务器响应：{}", response.trim());

            // 客户端发送 "bye" 请求关闭服务器 (根据之前的服务器逻辑)
            let close_message = "bye";
            stream.write(close_message.as_bytes())?;
            println!("发送关闭请求：{}", close_message.trim());
        }
        Err(e) => {
            eprintln!("连接服务器失败：{}", e);
        }
    }

    println!("客户端程序结束。");
    Ok(())
}