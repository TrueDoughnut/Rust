use std::io;

pub fn run(){
    println!("Enter the command");
    let mut entered = String::new();

    io::stdin().read_line(&mut entered)
        .expect("failed read line");

    let vec: Vec<&str> = entered.split(" ").collect();

    let last = match vec.last() {
       Some(num) => num,
        None(_) => "fail"
    };

    println!("{}", last);
}