#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
use core::panic::PanicInfo;
use core::fmt::Write;
use vga::{ColorChar, ScreenChar};
mod vga;


#[no_mangle] // don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    // welcome msg of the kernel
    welcome_kernel();
loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


fn welcome_kernel(){
    let mut writer = vga::WriteVGA {
        color: ColorChar::new_color(vga::Color::Green, vga::Color::Blue),
        line: 1,
        col: 1,
        vga_buff:  unsafe{&mut *(0xb8000 as *mut vga::Buffer)}
    };
    vga::ColorChar::fill_background(1);
     write!(writer, "
    DARK OS - Kernel\n\n
    Explicit is better than implicit
    Avoid unsafe code. Failing that, encapsulate it
    Things that need not change shouldn't
    Work with the type system
    This isn't finished

    Allocs:
    Writer->WriterVGA: {:p}",  &writer as *const _).unwrap();
}
