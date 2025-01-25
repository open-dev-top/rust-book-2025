#![allow(unused)]
fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let s = String::from("initial contents");


    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // 没有获得s2所有权，否则无法打印
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');


    // 只能将 &str 和 String 相加，不能将两个 String 值相加
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!"); // &String 可以被 强转（coerced）成 &str
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");


    let s1 = String::from("hello");
    // let h = s1[0]; // Rust 的字符串不支持索引


    let hello = String::from("Hola");
    let hello = String::from("Здравствуйте");

    let hello = "Здравствуйте";
    // （西里尔字母的 Ze）З 的第一个字节是 208，
    // 第二个是 151，所以 answer 实际上应该是 208，
    // 不过 208 自身并不是一个有效的字母。
    // let answer = &hello[0];


    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; // panic


    for c in "Зд".chars() {
        println!("{c}");
    }
    // 从字符串中获取如同天城文这样的字形簇是很复杂的，所以标准库并没有提供这个功能
    for b in "Зд".bytes() {
        println!("{b}");
    }
    
}
