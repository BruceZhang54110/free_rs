fn main() {
    let v1 = vec![1, 2, 3];
    // 创建了一个迭代器，这段代码本身并没有执行任何有用的操作。
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
