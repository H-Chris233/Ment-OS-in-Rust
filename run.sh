#!/bin/bash
# Run MentOS in QEMU

set -e

echo "Building kernel..."
cargo build --release

echo "Creating bootable image..."
cargo bootimage --release

echo "Starting QEMU..."
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-ment_os/release/bootimage-ment_os.bin \
    -serial mon:stdio \
    -display gtk \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    "$@"
