#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate rand;

use self::rand::Rng;
use std::fs::File;
use std::io::prelude::*;

pub fn run(){
    let mut file = File::open("src\\dice_roller\\data.txt")
        .expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong with reading the file");

    let split = contents.split("\n");

    let mut stack = Vec::new();

    for s in split {
        let nums: Vec<&str> = s.split("d").collect();

        let a: i32 = match nums[0].trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };
        let b: i32 = match nums[1].trim().parse() {
           Ok(num) => num,
            Err(_) => 0
        };

        let mut sum: i32 = 0;
        for i in (1..a).rev() {
            sum += rand::thread_rng().gen_range(1, b);
        }
        stack.push(sum);
    }

    for i in stack.iter() {
        println!("{}", i);
    }
}