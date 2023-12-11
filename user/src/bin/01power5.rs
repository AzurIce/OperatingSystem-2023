#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{get_time, yield_};

const LEN: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let p = 5u64;
    let m = 998244353u64;
    let iter: usize = 140000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            println!("power_5 [{}/{}]", i, iter);
            let current_timer = get_time();
            let wait_for = current_timer + 50;
            while get_time() < wait_for {
                yield_();
            }
        }
    }
    println!("{}^{} = {}", p, iter, s[cur]);
    println!("Test power_5 OK!");
    0
}