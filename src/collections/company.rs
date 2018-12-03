use std::io;
use std::collections::HashMap;

pub fn run(){

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut entered = String::new();

    while entered != "exit" {
        println!("Enter the command");

        io::stdin().read_line(&mut entered)
            .expect("Failure"); ;

        let vec: Vec<&str> = entered.split(" ").collect();

        let last = match vec.last() {
            Some(word) => word,
            None => panic!(),
        };

        if vec[0] == "add" {
            let name = vec[1];
            if map.contains_key(last) {
                let inner = match map.get_mut(last) {
                    Some(vec) => vec,
                    None => panic!(),
                };
                inner.push(name);
            } else {
                map.insert(last, vec!(vec[1]));
            }
        } else if vec[0] == "remove" {}
    }
    println!("{:?}", map);
}