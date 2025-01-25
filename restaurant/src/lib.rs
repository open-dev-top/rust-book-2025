use std::collections::HashMap;
use std::fmt;
// use std::io;
use std::io::Result as IoResult;


// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};


// use std::io;
// use std::io::Write;
use std::io::{self, Write};

use std::collections::*; // Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// use crate::front_of_house::hosting; // 外部代码使用 restaurant::front_of_house::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting; // 外部代码使用restaurant::hosting::add_to_waitlist

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();


    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");


    let order1 = back_of_house::Appetizer::Soup;
    let order2 = super::Appetizer::Salad;


    hosting::add_to_waitlist();


    let mut map = HashMap::new();
    map.insert(1, 2);
}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}


    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }


    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn function1() -> fmt::Result {
    // --snip--
}

// fn function2() -> io::Result<()> {
//     // --snip--
// }

fn function2() -> IoResult<()> {
    // --snip--
}