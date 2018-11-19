#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod guessing_game;
mod fibonacci;
mod dice_roller;

fn main() {
    //guessing_game::run();
    //expression();
    //if_statements();
    //loops();
    //fibonacci::run();
    dice_roller::run();
}


fn expression(){
    let x = 5;

    let y = {
        let x = 3;

        x + 1
    };

    println!("The value of y is: {}", y);
}

fn if_statements(){
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of the number is: {}", number);
}

fn loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result == 20);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter(){
        println!("The value is: {}", element);
    }
}