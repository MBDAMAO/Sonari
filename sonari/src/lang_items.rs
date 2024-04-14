use core::panic::PanicInfo;

use crate::{println, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loaction) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            loaction.file(),
            loaction.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown(true);
}
