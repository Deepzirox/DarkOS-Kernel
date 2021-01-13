#!/usr/bin/bash
cargo bootimage --target x86_64-kernel.json
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kernel/debug/bootimage-os.bin
