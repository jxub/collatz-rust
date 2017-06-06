extern crate rand;

use std::string;
use rand::Rng;

fn main() {
    println!("Let's solve the Collatz conjencture!");
    collatz();
}

fn collatz() {
    let mut foo = Data::new();
    while foo.not_finished() {
        if foo.is_even() {
            foo.change_even();
        }
        else {
            foo.change_odd();
        }
    }
    println!("{}", foo.status());
}

struct Data {
    num: i32,
    steps: i32,
}

impl Data {
    pub fn new() -> Data {
        Data {
            num: rand::thread_rng().gen_range(2, 100) as i32,
            steps: 0,
        }
    }
    pub fn add_step(&mut self) {
        self.steps += 1;
    }
    pub fn change_even(&mut self) {
        self.num = self.num / 2;
        Data::add_step(self);
    }
    pub fn change_odd(&mut self) {
        self.num = 3 * self.num + 1;
        Data::add_step(self);
    }
    pub fn not_finished(&mut self) -> bool {
        if self.num != 1 {
            true
        } else {
            false
        }
    }
    pub fn status(&mut self) -> string::String {
        format!("{} steps to reach 1", self.steps)
    }
    pub fn is_even(&mut self) -> bool {
        if self.num % 2 == 0 {
            true
        } else {
            false
        }
    }
}