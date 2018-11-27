#![allow(dead_code)]
#![allow(unused_variables)]

pub mod Coins;
use self::Coins::value_in_cents;
use self::Coins::Coin;
use self::Coins::State;

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn run(){
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{}", value_in_cents(Coin::Quarter(State::Illinois)));
}