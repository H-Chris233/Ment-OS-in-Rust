#![no_std]
#![no_main]
use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println("Hello World!");
    println("This is a simple kernel written in Rust.");
    loop {}
    
}

fn print(s: &str) {
    for byte in s.bytes() {
        print_byte(byte);
    }
}

fn println(s: &str) {
    print(s);
    print_byte(b'\n');
}

fn print_byte(byte: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    static mut COLUMN: u32 = 0;
    static mut ROW: u32 = 0;

    unsafe {
        match byte {
            b'\n' => {
                ROW += 1;
                COLUMN = 0;
            }
            byte => {
                let color_byte = 0xb;
                let row = ROW;
                let column = COLUMN;

                let offset = 2 * (row * 80 + column);
                *vga_buffer.offset(offset as isize) = byte;
                *vga_buffer.offset(offset as isize + 1) = color_byte;

                COLUMN += 1;
            }
        }
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

