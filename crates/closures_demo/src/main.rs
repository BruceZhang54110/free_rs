#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_perference: Option<ShirtColor>) -> ShirtColor {
        // 这里是无参闭包的写法
        // unwrap_or_else 函数接受一个闭包作为参数，当Option的值是None时，会调用这个闭包
        user_perference.unwrap_or_else(|| self.most_stocked())
    }
    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }  
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }

    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with perference {:?} get {:?}", user_pref1, giveaway1);


    let user_pref2 = None;

    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with perference {:?} get {:?}", user_pref2, giveaway2);


    let e = |x| x;
    let s = e(String::from("hello"));
    let n = e(5);

}
