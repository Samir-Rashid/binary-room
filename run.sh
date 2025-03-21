#!/usr/bin/env bash

ARCH=$1
ASM_FILE=$2
PLATFORM=""
LD_FLAGS=""
BENCHMARKING="true" # "true" to enable
QEMU=""

if [[ -z "$ARCH" || -z "$ASM_FILE" ]]; then
    echo "Error: Architecture and assembly (.S) file must be provided."
    echo "Usage: ./run.sh [riscv|arm] <assembly_file.S>"
    exit 1
fi

if [[ "$ARCH" != "riscv" && "$ARCH" != "arm" ]]; then
    echo "Error: Architecture must be either 'riscv' or 'arm'."
    echo "Usage: ./run.sh [riscv|arm] <assembly_file.S>"
    exit 1
fi

if [[ "$ARCH" == "riscv" ]]; then
    QEMU="qemu-riscv64"
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        PLATFORM="riscv64-unknown-linux-gnu-"
    fi
elif [[ "$ARCH" == "arm" ]]; then
    QEMU="qemu-aarch64"
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        PLATFORM="aarch64-unknown-linux-gnu-"
    fi
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

