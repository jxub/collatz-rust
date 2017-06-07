mod lib;

use lib::Data;

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
