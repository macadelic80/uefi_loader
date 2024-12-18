#![no_main]
#![no_std]

use r_efi::{
    efi,
    protocols::simple_text_input::InputKey
};

mod protocols;
use protocols::{logger::log, console::text_output::TextOutput};
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let mut x = 0;
    let text_output: TextOutput = TextOutput::new(st);
    loop {
        let r = Logln!(&text_output, "Appuyez sur une touche");
        if r.is_error() {
            return r;
        }
        let r = unsafe {
            ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut x)
        };
        if r.is_error() {
            return r;
        }
        let mut key = InputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        let r = unsafe { ((*(*st).con_in).read_key_stroke)((*st).con_in, &mut key) };
        if r.is_error() {
            return r;
        }
        if key.unicode_char != 0 {
            let r =Log!(&text_output, "Touche unicode: {}\n", key.unicode_char as u8 as char);
            if r.is_error() {
                return r;
            }
        } else {
            let r = Log!(&text_output, "Touche non reconnue: {}\n", key.scan_code);
            if r.is_error() {
                return r;
            }
            if r.is_error() {
                return r;
            }
        }
    }
}