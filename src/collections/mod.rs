use std::collections::HashMap;
use std::f32;

mod vector;
mod hash_map;

pub fn run(){
    vector::run();

    hash_map::run();

    let vec = [30, 30, 3892, 3893, 50, 38, 2, 43, 689, 390].to_vec();
    stats(vec);
}

fn stats(arr: Vec<i32>){
    println!("Average: {}", average(&arr));
    println!("Median: {:?}", median(&arr));
    println!("Mode: {}", mode(&arr));
}

fn average(arr: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }

    return sum as f32 / arr.len() as f32;
}

fn mode(arr: &Vec<i32>) -> i32 {
    assert!(arr.len() > 0);

    let mut map = HashMap::new();
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let num = match arr.get(0) {
        Some(num) => num,
        None => {
            println!("panic");
            panic!();
        }
    };

    let max = match map.get(num){
        Some(num) => *num,
        None => {
            println!("panic");
            panic!();
        }
    };

    let mut max_key: i32 = -1;
    for (key, value) in map {
        if value > max {
            max_key = *key;
        }
    }
    return max_key;
}

#[derive(Debug)]
enum Result {
    Decimal(f32),
    Integer(i32),
}

fn median(arr: &Vec<i32>) -> Result {
    let mut vec = arr.clone();
    vec.sort();

    if vec.len() % 2 == 0 {
        let mid = vec.len() / 2;
        let first: f32 = arr[mid] as f32;
        let second: f32 = arr[mid+1] as f32;
        let result = Result::Decimal((first+second)/ 2.0);
        return result;
    } else {
        let mid = vec.len() / 2;
        return Result::Integer(vec[mid]);
    }
}