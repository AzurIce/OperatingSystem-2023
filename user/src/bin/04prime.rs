#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const MAX: usize = 100;

fn is_prime(x: usize) -> bool {
    let mut cnt = 0;
    for i in 1..=x {
        if x % i == 0 {
            cnt += 1;
        }
    }
    cnt == 2
}

#[no_mangle]
fn main() -> i32 {
    for i in 2..=MAX {
        if is_prime(i) {
            print!("{i} ");
        }
    }
    println!("prime done!");
    0
}