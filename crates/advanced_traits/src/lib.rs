use std::{fmt, ops::Add};
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
    
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human flying");
    }
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!(" {} ", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));

    }
}


impl OutlinePrint for Point {
    
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// newtype pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }

}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }
}

impl Wrapper {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_add() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1 + p2;
        assert_eq!(p3, Point { x: 4, y: 6 });
    }

    #[test]
    fn test_add_millimeters() {
        let m1 = Millimeters(100);
        let m2 = Meters(1);

        let m3 = m1 + m2; // m2 + m1 就会报错
        assert_eq!(m3, Millimeters(1100));
    }

    #[test]
    fn test_fly() {
        let human = Human;
        human.fly(); // 调用 Human 的 fly 方法
        Pilot::fly(&human); // 调用 Pilot 的 fly 方法
        Wizard::fly(&human); // 调用 Wizard 的 fly 方法

    }

    #[test]
    fn test_baby_name() {

        // 不相等
        assert_ne!(Dog::baby_name(), String::from("Puppy"));
        // 相等
        assert_eq!(Dog::baby_name(), String::from("Spot"));
        // 相等
        assert_eq!(<Dog as Animal>::baby_name(), String::from("Puppy"));


    }

    #[test]
    fn test_display() {
        let p1 = Point {
            x: 1,
            y: 3
        };
        p1.outline_print();

    }

    #[test]
    fn test_newtype() {

        let mut w = Wrapper(vec![
            String::from("hello"),
            String::from("world"),
        ]);
        println!("{}", w);

        assert_eq!(w.len(), 2);
        w.push(String::from("rust"));

        println!("{}", w);

    }

}