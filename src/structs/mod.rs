#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

pub mod tuple_struct;
pub mod area;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn run(){
    let mut user1 = User {
       username: String::from("Dog"),
        email: String::from("example@gmail.com"),
        sign_in_count: 1,
        active: true
    };

    println!("{}", user1.username);

    user1.username = String::from("Cat");

    println!("{}", user1.username);

    let user2 =
        build_user(String::from("test@gmail.com"),
                   String::from("Golden Retriever"));

    println!("{}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}