#![allow(unused)]

struct User {
    active: bool,
    // username: &str, // 需要生命周期，见第10章
    username:String,
    // email: &str,
    email:String, // 需要生命周期，见第10章
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        // username: "someusername123",
        username:String::from("someusername123"),
        // email: "someone@example.com",
        email:String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
