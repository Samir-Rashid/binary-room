#!/usr/bin/env bash

ASM_FILE=$1


aarch64-linux-gnu-as $ASM_FILE -o $ASM_FILE.as
aarch64-linux-gnu-ld $ASM_FILE.as -o $ASM_FILE.bin
./$ASM_FILE.bin
echo $?
