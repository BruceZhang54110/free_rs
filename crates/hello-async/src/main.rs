use trpl::Html;
use trpl::Either;


async fn page_title(url: &str) -> (&str, Option<String>) {
    // 传入的任意 URL，使用 await 等待响应，因为Rust的futures是惰性的，只有调用await时，才会执行异步操作
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
    .select_first("title")
    .map(|title_element| title_element.inner_html());
    (url, title)
}
 
fn main() {
    // 接收参数，两个参数分别是两个URL
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
        
    });
}
