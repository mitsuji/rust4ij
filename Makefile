#!/bin/sh

BAS2BIN="../../c/bas2bin_for_IchigoJam/bas2bin"
LPC21ISP="../../c/lpc21isp_197k/lpc21isp"
USBSERIAL="/dev/tty.SLAB_USBtoUART"

DST=target/thumbv6m-none-eabi/release

all: build write

build:
	cargo build --release
	arm-none-eabi-objcopy -O ihex ${DST}/rust4ij ${DST}/rust4ij.hex
	arm-none-eabi-objcopy -O binary ${DST}/rust4ij ${DST}/rust4ij.bin

clean:
	cargo clean

write:
	echo "1 @ARUN:POKE#800,3,180,4,73,8,104,64,28,0,209,3,73,1,49,140,70,3,188,96,71,0,100,0,0,0,116,0,0:?USR(#800)" > ${DST}/entry.bas
	${BAS2BIN} ${DST}/entry.bas ${DST}/entry.bin
	cat ${DST}/entry.bin ${DST}/rust4ij.bin > ${DST}/sector6.bin
	${LPC21ISP} -control -bin -sector6 ${DST}/sector6.bin ${USBSERIAL} 115200 1200
