#![no_std]
#![no_main]

extern crate panic_halt;

use rust4ij::std15::*;

#[link_section = ".main"]
#[no_mangle]
fn main(_param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32, u32) -> u64) -> i32 {
	cls();
	let mut x:u32 = 15;
	let mut score:u32 = 0;
	loop {
		locate(x, 5);
		putc(236 as char);
		locate(rnd(32), 23);
		putc('*');
		putc(10 as char);
		wait(3);
		let c:u32 = inkey();
		if c == 28 {
			x -= 1;
		}
		if c == 29 {
			x += 1;
		}
		if scr(x, 5) != 0 {
			break;
		}
		score += 1;
	}
	putstr(b"SCORE:\0" as *const u8);
	putnum(score);
	putc(10 as char);
	return 0;
}
