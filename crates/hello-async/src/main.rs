use trpl::Html;


async fn page_title(url: &str) -> Option<String> {
    // 传入的任意 URL，使用 await 等待响应，因为Rust的futures是惰性的，只有调用await时，才会执行异步操作
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
    .select_first("title")
    .map(|title_element| title_element.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The tile for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}
