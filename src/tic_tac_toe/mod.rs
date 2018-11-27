extern crate rand;

use self::rand::Rng;

pub fn run(){
    let board = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

}

fn ai() -> u32 {
    rand::thread_rng().gen_range(0, 2)
}

