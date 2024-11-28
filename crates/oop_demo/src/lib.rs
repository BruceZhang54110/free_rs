pub struct AveragedCollection {
    // 集合
    list: Vec<i32>,
    // 平均数
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c = AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        };
        c.add(1);
        c.add(5);
        println!("average value: {}", c.average());
        c.remove();
        println!("remove a node, and now average value: {}", c.average());


    }

}