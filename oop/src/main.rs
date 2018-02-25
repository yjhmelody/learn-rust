pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
// 注意，结构体自身被标记为 pub ，这样其他代码可以使用这个结构体，但是在结构体内部的字段仍然是私有的。

impl AveragedCollection {
    pub fn new() -> Self {
        let avg = AveragedCollection{list: vec![], average:0f64};
        avg
    }

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
            },
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

fn main() {
    let mut avg = AveragedCollection::new();
    avg.add(1);
    avg.add(2);
    avg.add(3);
    println!("{}", avg.average);
}