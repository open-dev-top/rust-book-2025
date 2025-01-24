#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg 拥有参数的所有权
        height: 50,
    };

    // println!("rect1 is {}", rect1);
    // println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1); // dgb 不拥有参数的所有权

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
