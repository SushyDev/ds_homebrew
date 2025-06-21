#![no_main]
#![no_std]

extern crate alloc;
use core::ffi::*;
use libnds_sys::arm9_bindings::*;
use libnds_sys::video_registers::*;

mod assets;
use assets::*;

#[no_mangle]
extern "C" fn main() -> c_int {
    unsafe {
        videoSetMode(MODE_5_2D as u32);
        vramSetBankA(VRAM_A_MAIN_BG_0x06000000 as u32);
        consoleDemoInit();

        let bg_id = bgInit(
            3,
            BgType_Bmp8 as u32,
            BgSize_B8_256x256 as u32,
            0,
            0,
        );

        let bg_gfx_ptr = bgGetGfxPtr(bg_id) as *mut u16;

        dmaCopy(DOG_LOGO_BITMAP.data.as_ptr() as *const c_void, bg_gfx_ptr as *mut c_void, 65536);
        dmaCopy(DOG_LOGO_PALETTE.data.as_ptr() as *const c_void, BG_PALETTE as *mut c_void, 512);

        loop {
            swiWaitForVBlank();
        }
    }
}
