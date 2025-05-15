use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("无法绑定地址");
    println!("Server listening on 127.0.0.1:8080");

    let mut buffer = [0; 1024];
    loop {
        let (size, source) = socket.recv_from(&mut buffer).expect("接收数据失败");
        let request = String::from_utf8_lossy(&buffer[..size]);
        println!("Received request: {} from {}", request, source);

        let response = "Hello, client!";
        socket.send_to(response.as_bytes(), source).expect("发送响应失败");
    }
}