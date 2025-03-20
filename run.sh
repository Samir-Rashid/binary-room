#!/usr/bin/env bash

ASM_FILE=$1
PLATFORM=""
LD_FLAGS=""
BENCHMARKING="true" # "true" to enable
QEMU="qemu-riscv64"

if [[ -z "$ASM_FILE" ]]; then
    echo "Error: Assembly (.S) file is not passed in."
    echo "Usage: ./run.sh test_binary_translate_add.S"
    exit 1
fi

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="riscv64-unknown-linux-gnu-"
fi

if [[ "$OSTYPE" == "darwin"* ]]; then
    LD_FLAGS="-lSystem -macosx_version_min 11.3 -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib"
fi


"$PLATFORM"as "$ASM_FILE" -o "$ASM_FILE".as || { echo "Assembly compilation failed"; exit 1; }
"$PLATFORM"ld "$ASM_FILE".as -o "$ASM_FILE".bin $LD_FLAGS || { echo "Linking failed"; exit 1; }

"$QEMU" ./"$ASM_FILE".bin
echo "$?"

if [ "$BENCHMARKING" = true ]; then
    hyperfine -r 1000 -w 100 -Ni ""$QEMU" ./"$ASM_FILE".bin"
fi
