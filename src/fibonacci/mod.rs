#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io;

pub fn run(){
    println!("Enter a number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => 5,
    };

    println!("The {}th value of the fibonacci sequence is: {}", guess, fib(guess));
    println!("The {}th factorial is {}", guess, factorial(guess as i64));
}

fn fib(x: u32) -> u32 {
    if x <= 1 {
        x
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn factorial(x: i64) -> i64 {
    if x == 1 {
        1
    } else {
        factorial(x - 1) * x
    }
}