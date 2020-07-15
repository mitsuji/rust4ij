#![no_std]

pub mod std15 {
    pub fn rnd (x:u32) -> u32 {
	let addr_buff = 0xC0 as * const u16;
	let f: fn(u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn sin (x:u32) -> u32 {
	let addr_buff = 0xC2 as * const u16;
	let f: fn(u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn putc (x:char) {
	let addr_buff = 0xC4 as * const u16;
	let f: fn(char) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn putnum (x:u32) {
	let addr_buff = 0xC6 as * const u16;
	let f: fn(u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn putstr (x:*const u8) {
	let addr_buff = 0xC8 as * const u16;
	let f: fn(*const u8) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn inkey () -> u32 {
	let addr_buff = 0xCA as * const u16;
	let f: fn() -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f();
    }

    pub fn cls () {
	let addr_buff = 0xCC as * const u16;
	let f: fn() = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f();
    }

    pub fn locate (x:u32,y:u32) {
	let addr_buff = 0xCE as * const u16;
	let f: fn(u32,u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x,y);
    }

    pub fn scr (x:u32,y:u32) -> u32 {
	let addr_buff = 0xD0 as * const u16;
	let f: fn(u32,u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x,y);
    }

    pub fn pset (x:u32,y:u32) {
	let addr_buff = 0xD2 as * const u16;
	let f: fn(u32,u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x,y);
    }

    pub fn scroll (x:u32) {
	let addr_buff = 0xD4 as * const u16;
	let f: fn(u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn wait (x:u32) {
	let addr_buff = 0xD6 as * const u16;
	let f: fn(u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn out (x:u32,y:u32) {
	let addr_buff = 0xD8 as * const u16;
	let f: fn(u32,u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x,y);
    }

    pub fn r#in () -> u32 {
	let addr_buff = 0xDA as * const u16;
	let f: fn() -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f();
    }

    pub fn pwm (x:u32,y:u32,z:u32) {
	let addr_buff = 0xDC as * const u16;
	let f: fn(u32,u32,u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x,y,z);
    }

    pub fn ana (x:u32) -> u32 {
	let addr_buff = 0xDE as * const u16;
	let f: fn(u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn uputc (x:u32) {
	let addr_buff = 0xE0 as * const u16;
	let f: fn(u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(x);
    }

    pub fn memclear (dst:* mut u8,len:i32) {
	let addr_buff = 0xE4 as * const u16;
	let f: fn(* mut u8,i32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(dst,len);
    }

    pub fn memcpy (dst:* mut u8, src:* const u8,len:i32) {
	let addr_buff = 0xE6 as * const u16;
	let f: fn(* mut u8,* const u8,i32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(dst,src,len);
    }

    pub fn flash1 (cmd:u32, startsector:u32, endsector: u32) -> u32{
	let addr_buff = 0xE8 as * const u16;
	let f: fn(u32,u32,u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(cmd,startsector,endsector);
    }

    pub fn flash2 (cmd:u32, src:*const u8, dst:*mut u8,len:u32) -> u32{
	let addr_buff = 0xEA as * const u16;
	let f: fn(u32,*const u8,*mut u8,u32) -> u32 = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(cmd,src,dst,len);
    }

    pub fn ws_led (countrepeat:u32, data:*const u8, gpiomask:u32){
	let addr_buff = 0xEE as * const u16;
	let f: fn(u32,*const u8,u32) = unsafe { core::mem::transmute(*addr_buff as u32) };
	return f(countrepeat,data,gpiomask);
    }

}
