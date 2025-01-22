fn main() {
    println!("Hello, world!");

    another_function();
    // another_function(5); // rust does not support overload
    another_function_with_arg(5); // this 5 is called 'arguments'

    print_labeled_measurement(5, 'h');


    // let x = (let y = 6); // this is a statement 
    let y = {
        let x = 3;
        x + 1 // if add a ; at the end of this line, it will be changed to a statement
    };
    println!("The value of y is: {y}");


    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg(x: i32) { // this x is called 'parameters'
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
