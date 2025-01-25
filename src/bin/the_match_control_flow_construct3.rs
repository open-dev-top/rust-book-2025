fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // 如果注释这一行，匹配会不穷尽
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}
