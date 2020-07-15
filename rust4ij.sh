#!/bin/sh

BAS2BIN="../../c/bas2bin_for_IchigoJam/bas2bin"
LPC21ISP="../../c/lpc21isp_197k/lpc21isp"
USBSERIAL="/dev/tty.SLAB_USBtoUART"

case "$1" in
	"build" )
		cargo build --release
		cd target/thumbv6m-none-eabi/release
		arm-none-eabi-objcopy -O ihex rust4ij rust4ij.hex
		arm-none-eabi-objcopy -O binary rust4ij rust4ij.bin
		cd ../../../
		;;
	"clean" )
		cargo clean
		;;
	"write" )
		cd target/thumbv6m-none-eabi/release
		echo "1 @ARUN:POKE#800,3,180,4,73,8,104,64,28,0,209,3,73,1,49,140,70,3,188,96,71,0,100,0,0,0,116,0,0:?USR(#800)" > entry.bas
		${BAS2BIN} entry.bas entry.bin
		cat entry.bin rust4ij.bin > sector6.bin
		${LPC21ISP} -control -bin -sector6 sector6.bin ${USBSERIAL} 115200 1200
		cd ../../../
		;;
esac
