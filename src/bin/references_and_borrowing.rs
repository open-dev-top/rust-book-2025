#![allow(unused)]
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    let s = String::from("hello");

    // change(&s);


    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    println!("{r1}");
    let r2 = &mut s; 
    println!("{r2}");
    // println!("{}, {}", r1, r2);


    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;


    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    println!("{r1}");
    let r2 = &s; // 没问题
    println!("{r2}");
    let r3 = &mut s; // 大问题
    println!("{r3}");
    // println!("{}, {}, and {}", r1, r2, r3);

}

fn calculate_length(s: &String) -> usize { // s 是 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
