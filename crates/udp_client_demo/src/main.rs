use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("无法绑定地址");
    socket.connect("127.0.0.1:8080").expect("无法连接到服务器");
    
    loop {
        
        let response = "Hello, Server!";
        socket.send(response.as_bytes()).expect("发送失败");

        let mut buffer = [0; 1024];
        let (size, source) = socket.recv_from(&mut buffer).expect("接收数据失败");
        let request = String::from_utf8_lossy(&buffer[..size]);
        println!("Received request: {} from {}", request, source);
    }
}