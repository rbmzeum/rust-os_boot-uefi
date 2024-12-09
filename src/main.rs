#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod uefi;
use uefi::*;
#[no_mangle]
pub extern "efiapi" fn efi_main(_handle: ImageHandle, system_table: *const SystemTable) {
    unsafe {
        ((*(*system_table).output).clear_screen)((*system_table).output);
        ((*(*system_table).output).set_attribute)((*system_table).output, 0x0E); // 0 - чёрный фон, E - жёлтый текст
    }

    let msg = "Hello\n\r";
    for character in msg.chars() {
        let mut buffer: [u16; 1] = [0];
        let utf16 = character.encode_utf16(&mut buffer);
        unsafe {
            let _status =
                ((*(*system_table).output).output_string)((*system_table).output, &utf16[0]);
        }
    }

    //let string_arr = ['h' as u16, 'i' as u16, '!' as u16, '\n' as u16, '\0' as u16];
    let string_arr = [
        'W' as u16, 'o' as u16, 'r' as u16, 'l' as u16, 'd' as u16, '!' as u16,
    ];

    unsafe {
        let _status =
            ((*(*system_table).output).output_string)((*system_table).output, &string_arr[0]);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
