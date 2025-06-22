use std::time::Duration;

fn main() {
    trpl::run(async {
            let handle = trpl::spawn_task(
                async {
                    for i in 1..10 {
                        println!("hi numnber {i} from the first task!");
                        trpl::sleep(Duration::from_millis(500)).await;
                    }
                }
            );
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;            
            }
            handle.await.unwrap();

            

        }
    );
}