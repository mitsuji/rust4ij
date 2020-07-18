#![no_std]
#![no_main]

use rust4ij::std15::*;

#[link_section = ".main"]
#[no_mangle]
fn main(_param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32, u32) -> u64) -> i32 {
    putstr(b"a\n\0" as *const u8);
    putstr(b"ab\n\0" as *const u8);
    putstr(b"abc\n\0" as *const u8);
    putstr(b"abcd\n\0" as *const u8);
    putstr(b"abc\n\0" as *const u8);
    putstr(b"ab\n\0" as *const u8);
    putstr(b"a\n\0" as *const u8);
    return 0;
}

