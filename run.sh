#!/usr/bin/env bash

aarch64-linux-gnu-as test_binary_translate_add.S 
aarch64-linux-gnu-ld a.out -o a.bin
./a.bin
echo $?
