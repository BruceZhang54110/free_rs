use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // 发送放到一个future 中
        let tx_future = async {
            let vals = vec![
                String::from("Hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_future = async {
            while let Some(value) = rx.recv().await {
                println!("received: {value}");
            }
        };
        // 使用 join 接收两个future，返回一个新的future
        trpl::join(tx_future, rx_future).await;
    });


}