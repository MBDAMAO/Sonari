#![no_main]
#![no_std]
#![feature(panic_info_message)]
#[macro_use]
mod sbi;
mod console;
mod lang_items;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// prevent fn name change while compile
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("\x1b[31mhello world\x1b[0m");
    info!("what1?");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
