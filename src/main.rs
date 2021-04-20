use back_of_house::{Appetizer, Breakfast};
use std::collections::HashMap;

// 使用 pub use 重导出
// 通过 glob 运算符将所有的公有定义引入作用域
pub use front_of_house::*;

// 具有两个相同的名称的Result
// use std::fmt;
// use std::io;

// 使用 as 关键字重定义映入的名称
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// 嵌套路径来消除大量的 use 行
// use std::io::{self, Write};

mod a;
pub use crate::a::car::{self, Car, CarType};

fn main() {
    // ----- 模块 -----

    crate::front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();
    crate::front_of_house::serving::take_order();

    let mut meal = crate::back_of_house::Breakfast::summer("Toast Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // private 报错
    // meal.seasonal_fruit = String::from("blueberries");

    let soup_order = crate::back_of_house::Appetizer::Soup;
    let salad_order = crate::back_of_house::Appetizer::Salad;

    println!("soup order: {:?}", soup_order);
    println!("salad order: {:?}", salad_order);

    // 使用use后
    let soup = Appetizer::Soup;
    println!("soup order: {:?}", soup);
    let mut meal = Breakfast::summer("Toast");
    meal.toast = String::from("Hello");
    println!("meal.toast: {}", meal.toast);

    let mut map = HashMap::new();
    map.insert(String::from("age"), 29);
    println!("map: {:?}", map);

    let mut suv = Car::new(CarType::SUV);
    suv.length = 4.98;
    suv.width = 1.97;
    suv.height = 2.65;
    let mut mvp = Car::new(CarType::MVP);
    mvp.length = 5.53;
    mvp.width = 1.98;
    mvp.height = 2.79;
    println!("suv: {:?}", suv);
    println!("mvp: {:?}", mvp);
}

// fn result1() -> fmt::Result {}

// fn result2() -> io::Result {}

// fn result1() -> FmtResult {}

// fn result2() -> IoResult {}

mod back_of_house {

    // 枚举中的成员默认是共有的
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // 结构体成员默认是私有的
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 因为 back_of_house::Breakfast 具有私有字段，所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house {

    pub mod hosting {

        pub fn add_to_waitlist() {
            println!("front_of_house::hosting.add_to_waitlist()");
        }

        pub fn seat_at_table() {
            add_to_waitlist();
            super::serving::take_order();
        }
    }

    pub mod serving {

        pub fn take_order() {
            println!("front_of_house::hosting.take_order()");
        }

        fn server_order() {}

        fn take_payment() {}
    }
}
