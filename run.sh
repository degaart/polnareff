#!/usr/bin/env bash

qemu-system-i386 -debugcon stdio -no-reboot -device isa-debug-exit,iobase=0xf4,iosize=0x04 -kernel "$1"
STATUS=$?

if [ $STATUS -eq 3 ]; then
    echo "QEMU exited normally"
    exit 0
else
    echo "QEMU exited with status $STATUS"
    exit $STATUS
fi

