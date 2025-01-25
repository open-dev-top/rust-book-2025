#![allow(unused)]
// enum Option<T> {
//     None,
//     Some(T),
// } // 和标准库中的同名枚举冲突

fn main() {
    let some_number: Option<i32> = Some(5);
    let some_number2: Option<i32> = Some(6);
    // let some_number3: Option<i32> = some_number + some_number2;

    let some_char = Some('e');

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}
