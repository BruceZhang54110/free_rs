/// struct 的生命周期标注示例
pub struct ImportRxcerpt<'a> {
    // 表示该字段必须要和struct 这个实例存活时间一样长
    pub part: &'a str,
}

impl<'a> ImportRxcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
