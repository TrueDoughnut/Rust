#![allow(dead_code)]
#![allow(unused_variables)]

pub mod match_test;

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn run(){
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}