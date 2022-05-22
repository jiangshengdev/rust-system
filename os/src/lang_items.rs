use core::panic::PanicInfo;

use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = info.message().unwrap();

    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        error!("[kernel] Panicked: {}", err);
    }

    shutdown()
}
