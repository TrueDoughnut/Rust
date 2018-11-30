use std::collections::HashMap;
use std::f32;
use std::fmt;

mod vector;
mod hash_map;
mod company;

pub fn run(){
    //vector::run();

    //hash_map::run();

    //let vec = [30, 30, 3892, 3893, 50, 38, 2, 43, 689, 390].to_vec();
    //stats(vec);

    //println!("{}", pig_latin(String::from("that")));
    //println!("{}", pig_latin(String::from("apple")));

    company::run();
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
    let mut map = HashMap::new();
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    for (key, value) in map {
        if value == max_value {
            return *key;
        }
    }
    panic!("None");
}

enum Result {
    Decimal(f32),
    Integer(i32),
}

impl fmt::Debug for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Result::Decimal(num) => write!(f, "{}", num),
            Result::Integer(num) => write!(f, "{}", num),
        }
    }
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

fn pig_latin(word: String) -> String {
    let mut base: String = word.clone();

    let first: char = word.chars().next().unwrap();
    let val:String = match first {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            base.push_str(" hay");
            base
        },
        _ => {
            let end = &base[1..];
            let str: String = format!("{} {}ay", end, first);
            str
        }
    };

    return val;
}