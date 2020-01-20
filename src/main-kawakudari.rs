#![no_std]
#![no_main]

extern crate panic_halt;

use rust4ij::std15;

#[link_section = ".main"]
#[no_mangle]
fn main(_param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32,u32) -> u64) -> i32 {
    std15::cls();
    let mut x:i32 = 15;
    loop {
	std15::locate(x as u32, 5);
	std15::putc('0' as u32);
	std15::locate(std15::rnd(32), 23);
	std15::putc('*' as u32);
	std15::putc(10);
	std15::wait(3);
	let c:u32 = std15::inkey();
	if c == 28 {x=x-1}
	if c == 29 {x=x+1}
	if std15::scr(x as u32, 5) != 0 {break}
    }
    std15::putstr(b"GAME OVER\n\0" as *const u8);
    return x;
}

