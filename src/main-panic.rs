#![no_std]
#![no_main]

use rust4ij::std15::*;

#[link_section = ".main"]
#[no_mangle]
fn main(param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32,u32) -> u64) -> i32 {
    let mut ar:[i32; 4] = [1, 2, 3, 4];
    ar[(param + 4) as usize] = 100;
    let mut sum : i32 = 0;
    for i in 0..ar.len() {
        sum += ar[i];
    }
    return sum;
}
