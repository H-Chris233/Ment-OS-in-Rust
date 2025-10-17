#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use ment_os::{println, memory};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use memory::BootInfoFrameAllocator;

    println!("MentOS v0.1.0");
    println!("Initializing kernel...");

    ment_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let _mapper = unsafe { memory::init(phys_mem_offset) };
    let _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    println!("Kernel initialized successfully!");
    println!();
    println!("===========================================");
    println!("  Welcome to MentOS - A Minimal OS Kernel");
    println!("===========================================");
    println!();
    println!("Features:");
    println!("  [x] VGA Text Mode Driver");
    println!("  [x] Serial Port Output");
    println!("  [x] Interrupt Handling (IDT)");
    println!("  [x] Exception Handling");
    println!("  [x] Keyboard Input (PS/2)");
    println!("  [x] Memory Management (Paging)");
    println!("  [x] GDT & TSS");
    println!();
    println!("Type anything to test keyboard input...");
    println!();

    ment_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ment_os::hlt_loop();
}
