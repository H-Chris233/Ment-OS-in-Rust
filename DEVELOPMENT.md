# MentOS Development Guide

## Prerequisites

### Required Tools

1. **Rust Nightly Toolchain**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
source "$HOME/.cargo/env"
```

2. **Rust Components**
```bash
rustup component add rust-src llvm-tools-preview
```

3. **Bootimage Tool**
```bash
cargo install bootimage
```

4. **QEMU (Optional, for testing)**
```bash
# Ubuntu/Debian
sudo apt install qemu-system-x86

# macOS
brew install qemu

# Arch Linux
sudo pacman -S qemu
```

## Building

### Debug Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Create Bootable Image
```bash
# Debug
cargo bootimage

# Release
cargo bootimage --release
```

The bootable image will be created at:
- Debug: `target/x86_64-ment_os/debug/bootimage-ment_os.bin`
- Release: `target/x86_64-ment_os/release/bootimage-ment_os.bin`

## Running

### Using QEMU

**Manual Launch**:
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ment_os/debug/bootimage-ment_os.bin
```

**With Serial Output**:
```bash
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-ment_os/debug/bootimage-ment_os.bin \
    -serial mon:stdio
```

**Using Run Script**:
```bash
./run.sh
```

### Using Real Hardware

1. Write the bootimage to a USB drive:
```bash
# WARNING: This will erase all data on the USB drive!
# Replace /dev/sdX with your USB drive device
sudo dd if=target/x86_64-ment_os/release/bootimage-ment_os.bin of=/dev/sdX bs=4M
sudo sync
```

2. Boot from the USB drive (configure BIOS/UEFI to boot from USB)

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test
```bash
cargo test --lib test_name
```

## Development Workflow

### 1. Make Changes

Edit source files in `src/`:
- `main.rs`: Kernel entry point
- `lib.rs`: Library root
- Module files: `vga_buffer.rs`, `serial.rs`, etc.

### 2. Build

```bash
cargo build
```

Fix any compilation errors.

### 3. Test

```bash
cargo test
```

### 4. Run

```bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ment_os/debug/bootimage-ment_os.bin
```

## Common Development Tasks

### Adding a New Module

1. Create the module file: `src/my_module.rs`

2. Declare in `src/lib.rs`:
```rust
pub mod my_module;
```

3. Use in `src/main.rs`:
```rust
use ment_os::my_module;
```

### Adding a New Interrupt Handler

1. Add to IDT in `src/interrupts.rs`:
```rust
idt[InterruptIndex::MyInterrupt.as_usize()]
    .set_handler_fn(my_interrupt_handler);
```

2. Implement the handler:
```rust
extern "x86-interrupt" fn my_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // Handle interrupt
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::MyInterrupt.as_u8());
    }
}
```

### Adding a New Test

```rust
#[test_case]
fn test_something() {
    assert_eq!(2 + 2, 4);
}
```

### Debugging Tips

1. **Serial Output**: Use `serial_println!` for debugging without affecting VGA
```rust
serial_println!("Debug: value = {}", value);
```

2. **QEMU Monitor**: Press `Ctrl+A` then `C` to access QEMU monitor

3. **Breakpoint Exception**: Use `int3!()` to trigger a breakpoint

4. **View Registers**: In QEMU monitor, use `info registers`

5. **View Memory**: In QEMU monitor, use `x/10i $rip` to disassemble

## Code Style Guidelines

### Naming Conventions
- **Types**: `PascalCase` (e.g., `ColorCode`, `ScreenChar`)
- **Functions**: `snake_case` (e.g., `write_byte`, `new_line`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `BUFFER_WIDTH`, `PIC_1_OFFSET`)
- **Static variables**: `SCREAMING_SNAKE_CASE` (e.g., `WRITER`, `SERIAL1`)

### Module Organization
- One primary type/abstraction per module
- Related helper types in the same module
- Public API should be minimal and well-documented

### Safety
- Minimize `unsafe` blocks
- Document why each `unsafe` block is necessary
- Use safe abstractions over unsafe code

### Comments
- Document public APIs
- Explain non-obvious design decisions
- Don't comment obvious code

## Troubleshooting

### Build Errors

**Error: "can't find crate for `core`"**
- Solution: Ensure `rust-src` component is installed
```bash
rustup component add rust-src
```

**Error: "target-pointer-width: invalid type"**
- Solution: Ensure numeric value in `x86_64-ment_os.json`:
```json
"target-pointer-width": 64
```

**Error: "SSE register return with SSE disabled"**
- Solution: Remove `-sse,+soft-float` from target features

### Runtime Issues

**Triple Fault (QEMU resets)**
- Check GDT is properly loaded
- Verify IDT is initialized before enabling interrupts
- Ensure interrupt handlers use correct ABI

**No Output**
- Verify VGA buffer address (0xb8000)
- Check that WRITER is properly initialized
- Ensure writes are volatile

**Keyboard Not Working**
- Verify PIC is initialized
- Check interrupt 33 (IRQ 1) is enabled
- Ensure EOI is sent after handling

## Performance Optimization

### Debug vs Release

Debug builds include:
- Debug symbols
- No optimizations
- Larger binary size

Release builds include:
- Full optimizations (`opt-level = 3`)
- Stripped debug info
- Smaller binary size

### Profile-Guided Optimization

For maximum performance:
```toml
[profile.release]
panic = "abort"
lto = true
codegen-units = 1
```

## Contributing

### Before Submitting

1. Ensure code compiles: `cargo build`
2. Run all tests: `cargo test`
3. Format code: `cargo fmt`
4. Check for warnings: `cargo clippy`
5. Test in QEMU
6. Update documentation if needed

### Code Review Checklist

- [ ] Code follows style guidelines
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] No unnecessary `unsafe` blocks
- [ ] Interrupt safety considered
- [ ] Memory safety verified

## Resources

### Learning Materials
- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Intel Software Developer Manuals](https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html)

### Rust Documentation
- [x86_64 crate](https://docs.rs/x86_64/)
- [bootloader crate](https://docs.rs/bootloader/)
- [volatile crate](https://docs.rs/volatile/)
- [lazy_static crate](https://docs.rs/lazy_static/)

### Tools
- [QEMU Documentation](https://www.qemu.org/documentation/)
- [GDB for Kernel Debugging](https://sourceware.org/gdb/documentation/)
- [Rust Analyzer](https://rust-analyzer.github.io/) (IDE support)

## FAQ

**Q: Why Rust?**
A: Memory safety, zero-cost abstractions, strong type system, excellent tooling.

**Q: Why x86_64?**
A: Widely supported, well-documented, powerful architecture.

**Q: Can I add features X?**
A: Yes! See the Future Enhancements section in ARCHITECTURE.md.

**Q: How do I debug kernel panics?**
A: Use serial output (`serial_println!`) and QEMU monitor commands.

**Q: Is this production-ready?**
A: No, this is an educational/experimental kernel. Don't use in production.

**Q: Can I run this on ARM?**
A: Not currently, but the architecture could be adapted with significant work.

**Q: How do I add heap allocation?**
A: Implement a heap allocator (buddy allocator, slab allocator, etc.) and register it with `#[global_allocator]`.

**Q: Can I boot from UEFI?**
A: Yes, the bootloader crate supports both BIOS and UEFI.
