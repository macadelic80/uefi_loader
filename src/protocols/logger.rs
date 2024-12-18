use core::fmt::{Write};
use r_efi::efi;

use super::{buffer_writer::BufferWriter, console::text_output::TextOutput};
fn str_to_utf16(s: &str) -> [u16; 128] {
    let mut buffer = [0u16; 128];
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= 127 {
            break;
        }
        buffer[i] = c;
        i += 1;
    }
    buffer[i] = 0; // Terminaison nulle
    buffer
}

pub fn log(to: &TextOutput, args: core::fmt::Arguments) -> efi::Status {
    let mut buffer = [0u8; 512]; // Taille fixe pour le buffer
    let mut writer = BufferWriter::new(&mut buffer);

    // Formatte les arguments dans le buffer
    if write!(&mut writer, "{}", args).is_err() {
        return efi::Status::BUFFER_TOO_SMALL;
    }

    let s = writer.written_data();

    (*to).output_string(str_to_utf16(s).as_ptr() as *mut efi::Char16)
}

/// Macro pour utiliser facilement la fonction `log`
#[macro_export]
macro_rules! Log {
    ($st:expr, $($arg:tt)*) => {{
        log($st, format_args!($($arg)*))
    }};
}


#[macro_export]
macro_rules! Logln {
    ($st:expr, $($arg:tt)*) => {{
        log($st, format_args!("{}\n", format_args!($($arg)*)))
    }};
}