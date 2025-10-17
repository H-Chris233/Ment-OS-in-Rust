# MentOS Architecture Documentation

## Overview

MentOS is a minimal, clean, high-quality operating system kernel written in Rust for the x86_64 architecture. It demonstrates modern OS kernel development practices with strong memory safety guarantees.

## System Architecture

### Boot Process

1. **Bootloader Stage** (bootloader crate 0.9.x)
   - BIOS/UEFI boot compatibility
   - Sets up initial page tables
   - Maps physical memory with offset
   - Loads kernel ELF binary
   - Jumps to kernel entry point

2. **Kernel Initialization** (`_start` via `entry_point!` macro)
   - VGA text buffer initialization
   - GDT (Global Descriptor Table) setup
   - IDT (Interrupt Descriptor Table) configuration
   - PIC (Programmable Interrupt Controller) initialization
   - Enable hardware interrupts
   - Memory paging initialization
   - Frame allocator setup

3. **Runtime Loop**
   - Enter HLT loop (CPU halts until next interrupt)
   - Handle timer interrupts
   - Process keyboard input
   - Handle exceptions (page faults, double faults, etc.)

### Memory Layout

```
Virtual Memory Map:
0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF : User space (not used)
0xFFFF_8000_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF : Kernel space
0x0000_0000_000B_8000 : VGA text buffer
Physical memory offset : Configured by bootloader
```

## Module Architecture

### 1. VGA Buffer Module (`vga_buffer.rs`)

**Purpose**: Hardware abstraction for VGA text mode (80x25 characters)

**Key Components**:
- `Color` enum: 16 colors (foreground/background)
- `ColorCode`: Combined foreground/background color
- `ScreenChar`: Character + color code pair
- `Buffer`: 2D array of screen characters
- `Writer`: Safe interface for writing to VGA buffer

**Features**:
- Automatic scrolling when buffer fills
- Color-coded output
- Volatile writes to prevent compiler optimization
- Thread-safe via spin mutex
- `print!` and `println!` macros

**Design Decisions**:
- Uses `volatile` crate to ensure writes are not optimized away
- Lazy static initialization for global writer instance
- Mutex-protected for safe concurrent access
- Implements `fmt::Write` trait for format string support

### 2. Serial Port Module (`serial.rs`)

**Purpose**: COM1 serial port for debugging output to QEMU

**Key Components**:
- `SERIAL1`: Global serial port instance (0x3F8)
- `_print`: Internal print function
- `serial_print!` and `serial_println!` macros

**Features**:
- 16550 UART driver
- Interrupt-safe output
- Compatible with QEMU serial redirection

**Usage**:
```rust
serial_println!("Debug: value = {}", value);
```

### 3. Global Descriptor Table Module (`gdt.rs`)

**Purpose**: Set up memory segmentation and task state segment

**Key Components**:
- `TSS` (Task State Segment): Separate stack for double fault handler
- `GDT`: Kernel code segment and TSS descriptor
- `Selectors`: Code and TSS segment selectors

**Design Decisions**:
- Separate interrupt stack (20 KiB) for double fault handler
- Prevents stack overflow from causing triple fault
- Uses IST (Interrupt Stack Table) index 0

### 4. Interrupt Handling Module (`interrupts.rs`)

**Purpose**: CPU exception and hardware interrupt handling

**Key Components**:
- `IDT`: Interrupt Descriptor Table
- Exception handlers: breakpoint, double fault, page fault
- Hardware interrupt handlers: timer, keyboard
- `PICS`: Dual 8259 PIC configuration

**Interrupt Vector Layout**:
```
0-31  : CPU exceptions
32    : Timer interrupt (PIC1, IRQ 0)
33    : Keyboard interrupt (PIC1, IRQ 1)
34-47 : Other hardware interrupts
```

**Features**:
- Proper EOI (End of Interrupt) signaling
- Interrupt-safe critical sections
- Stack trace on faults
- Graceful panic handling

### 5. Memory Management Module (`memory.rs`)

**Purpose**: Virtual memory paging and physical frame allocation

**Key Components**:
- `OffsetPageTable`: Page table walker
- `BootInfoFrameAllocator`: Physical frame allocator

**Features**:
- 4-level paging (PML4)
- Frame allocation from bootloader memory map
- Safe page table walking
- Physical memory mapping

**Memory Regions**:
- Usable: Available for allocation
- Reserved: Used by firmware/bootloader
- ACPI: ACPI tables
- Other: Various hardware regions

### 6. Keyboard Driver Module (`keyboard.rs`)

**Purpose**: PS/2 keyboard input handling

**Key Components**:
- `KEYBOARD`: Scancode decoder
- US104 layout support
- Scancode Set 1 handling

**Features**:
- Scancode to key event translation
- Unicode character output
- Special key handling (arrows, function keys, etc.)

**Flow**:
1. Hardware interrupt (IRQ 1) fires
2. Read scancode from port 0x60
3. Decode scancode to key event
4. Convert to Unicode character
5. Print to VGA buffer

## Synchronization & Concurrency

### Spinlocks

All global resources use spin mutexes:
- `WRITER` (VGA buffer)
- `SERIAL1` (serial port)
- `KEYBOARD` (keyboard state)
- `PICS` (interrupt controllers)

**Why Spinlocks?**
- No thread scheduler yet
- No blocking primitives
- Simple and deterministic
- Suitable for short critical sections

### Interrupt Safety

Critical sections use `without_interrupts`:
```rust
x86_64::instructions::interrupts::without_interrupts(|| {
    // Critical section
});
```

This prevents deadlocks from interrupt handlers accessing locked resources.

## Error Handling

### Panic Handler

```rust
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
```

- Prints panic message to VGA buffer
- Halts CPU (prevents undefined behavior)
- No unwinding (panic = abort)

### Exception Handlers

- **Breakpoint**: Debug trap, resumes execution
- **Double Fault**: Stack overflow or handler failure, halts
- **Page Fault**: Invalid memory access, prints address and error code

## Build System

### Custom Target Specification

`x86_64-ment_os.json`:
- Bare-metal target (no OS)
- Disables red zone (required for interrupts)
- Uses LLD linker
- Panic strategy: abort

### Cargo Configuration

`.cargo/config.toml`:
- Builds Rust core library from source
- Uses custom target by default
- Optional: bootimage runner for `cargo run`

### Build Process

1. `cargo build`: Compile kernel binary
2. `cargo bootimage`: Create bootable disk image
3. Bootimage wraps kernel with bootloader
4. Output: bootimage-ment_os.bin (bootable)

## Testing Infrastructure

### Test Framework

Custom test framework using Rust's unstable features:

```rust
#![feature(custom_test_frameworks)]
#![test_framework(crate::test_framework)]
```

### Test Harness

- Runs in no_std environment
- Uses QEMU exit device (port 0xf4)
- Reports success/failure via exit code
- Serial output for test results

### Example Test

```rust
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::int3(); // Should not panic
}
```

## Safety Considerations

### Memory Safety

- No raw pointer dereferencing except in clearly marked unsafe blocks
- Volatile access for hardware I/O
- Frame allocator prevents double allocation
- Page tables prevent out-of-bounds memory access

### Interrupt Safety

- Interrupt-safe mutexes
- Separate interrupt stack for fault handlers
- Proper PIC configuration and EOI signaling

### Type Safety

- Strong typing for colors, addresses, frames
- `#[repr(C)]` for hardware-mapped structures
- Zero-cost abstractions

## Performance Considerations

### Zero-Cost Abstractions

- Rust generics compiled away
- Inline functions where appropriate
- No runtime overhead from safety features

### Minimal Allocations

- Static allocation only (no heap yet)
- Lazy static initialization
- Fixed-size buffers

## Future Enhancements

Potential additions for a more complete OS:

1. **Heap Allocator**
   - Dynamic memory allocation
   - Buddy allocator or slab allocator

2. **Process Management**
   - Task switching
   - Scheduler
   - Context switching

3. **File System**
   - Virtual File System (VFS)
   - FAT32 or ext2 support

4. **Network Stack**
   - Ethernet driver
   - TCP/IP stack

5. **Shell**
   - Command interpreter
   - Built-in commands

6. **Multithreading**
   - Thread creation
   - Synchronization primitives

7. **User Mode**
   - System calls
   - User/kernel space separation

## References

- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [x86_64 crate documentation](https://docs.rs/x86_64/)
- [Intel® 64 and IA-32 Architectures Software Developer Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
