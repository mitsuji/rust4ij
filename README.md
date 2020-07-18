# Rust_for_IchigoJam

This is the Rust version of [c4ij](https://github.com/IchigoJam/c4ij).

## Minimum example

```
#![no_std]
#![no_main]

use rust4ij::std15;

#[link_section = ".main"]
#[no_mangle]
fn main(_param:i32, _ram:i32, _rom:i32, _divfunc: fn(u32,u32) -> u64) -> i32 {
    return std15::rnd(10) as i32;
}
```

## Prerequisite

* [GNU Arm Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads) (add it to your PATH environment variable.)
* [rustup](https://rustup.rs/) (add it to your PATH environment variable.)
* [bas2bin](https://github.com/IchigoJam/c4ij/blob/master/bas2bin.c) from [c4ij](https://github.com/IchigoJam/c4ij)
* [lpc21isp](https://github.com/taisukef/lpc21isp) customed by [taiskef](https://github.com/taisukef)


## Preparation

Update Rust environment to latest one.
```
$ rustup update
```

Append Cortex-M v6-M build target to your environment.
```
$ rustup target add thumbv6m-none-eabi
```

Adjust contents of rust4ij.sh to suit your environment.
```
BAS2BIN="../c4ij/bas2bin"
LPC21ISP="../lpc21isp/lpc21isp"
USBSERIAL="/dev/tty.SLAB_USBtoUART"
```


## How to use

To build & write src/main.rs
```
$ make
```

To build src/main.rs
```
$ make build
```

To write to IchigoJam
```
$ make write
```

To see disassembled program
```
$ make disasm
```

To clean project
```
$ make clean
```
