#![allow(unused)]
fn main() {
    // let guess: u32 = "42".parse();
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {guess}");

    
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is {x}");
    println!("The value of x is {y}");


    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1.66，舍入为 -1
    let truncated_positive = 5 / 3; // 结果为 1.66，舍入为 1
    println!("The value of quotient is {quotient}");
    println!("The value of truncated is {truncated}");
    println!("The value of truncated+ is {truncated_positive}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {remainder}");


    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is {t}");
    println!("The value of f is {f}");


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{},{},{}",c,z,heart_eyed_cat);


    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let a = [3; 5]; // five threes

}
