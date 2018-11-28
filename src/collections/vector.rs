pub fn run(){
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(38);

    let first: &i32 = &v[0];
    println!("{}", first);

    let v = vec![1, 2, 3, 4, 5];
    let v_index = 2;
    let x = match v.get(v_index) {
        Some(_) => {
            println!("Reachable element at index: {}", v_index);
            v[v_index]
        },
        None => {
            println!("Unreachable element at index: {}", v_index);
            0
        }
    };
    println!("Value: {}", x)
}