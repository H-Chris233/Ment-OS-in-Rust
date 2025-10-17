# MentOS - A Minimal Operating System Kernel

A clean, minimal, high-quality operating system kernel written in Rust for the x86_64 architecture.

## Features

- **VGA Text Mode Driver**: Color-coded text output with scrolling support
- **Serial Port Output**: COM1 serial output for debugging via QEMU
- **Interrupt Handling**: Complete IDT setup with exception handlers
- **Memory Management**: Basic paging support with frame allocator
- **Keyboard Input**: PS/2 keyboard driver with US104 layout
- **Global Descriptor Table**: Proper GDT and TSS configuration
- **Exception Handling**: Breakpoint, double fault, and page fault handlers

## Architecture

```
src/
├── main.rs           # Kernel entry point and initialization
├── lib.rs            # Library root for testing
├── vga_buffer.rs     # VGA text mode driver
├── serial.rs         # Serial port driver (COM1)
├── interrupts.rs     # IDT and interrupt handlers
├── gdt.rs            # Global Descriptor Table setup
├── memory.rs         # Memory management and paging
└── keyboard.rs       # Keyboard driver (PS/2)
```

## Building

Requires Rust nightly with the following components:

```bash
rustup default nightly
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
```

Build the kernel:

```bash
cargo build
```

Build bootable image:

```bash
cargo bootimage
```

## Running

Run in QEMU:

```bash
cargo run
```

Or with serial output:

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ment_os/debug/bootimage-ment_os.bin -serial mon:stdio
```

## Testing

Run tests:

```bash
cargo test
```

## Technical Details

- **Language**: Rust 2021 Edition
- **Architecture**: x86_64
- **Bootloader**: bootloader 0.9
- **Target**: Custom bare-metal x86_64 target
- **Memory Safety**: Enforced by Rust's ownership system
- **No Standard Library**: Pure `#![no_std]` environment

## License

MIT
