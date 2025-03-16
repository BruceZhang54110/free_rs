pub trait Draw {
    fn draw(&self);
}

/// `Vec<Box<dyn Draw>>` 在这里的用法含义：无法再编译期确定单一类型，就要使用智能指针在堆上分配
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 为结构体Screen 实现一个方法
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button drawing")
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox drawing");
    }
    
}
