#![no_std]
#![no_main]

extern crate panic_halt;

use rust4ij::std15::*;

#[link_section = ".main"]
#[no_mangle]
fn main(_param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32,u32) -> u64) -> i32 {
        out(7, 1);
	return 0;
}
