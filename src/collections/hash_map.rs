use std::collections::HashMap;

pub fn run(){
    //create standard hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //create hashmap from vector
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //inserting if key has no value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores)
}