
#![no_main]
#![no_std]

extern crate alloc;
use core::ffi::*;
use libnds_sys::arm9_bindings::*;
use libnds_sys::println;
use libnds_sys::video_registers::*;

mod assets;
use assets::*;

use libm::ceilf;

fn f32_to_s32(value: f32) -> s32 {
    ceilf(value * 256.0) as s32
}

fn sprite_code() {
    unsafe {
        videoSetMode(MODE_0_2D as u32);
        vramSetBankA(VRAM_A_MAIN_SPRITE as u32);

        let mut oam_main: OamState = OamState::new();

        oamInit(oam_main, SpriteMapping_1D_32 as u32, false);

        let sprite_gfx_ptr = oamAllocateGfx(
            oam_main,
            SpriteSize_16x16 as u32,
            SpriteColorFormat_256Color as u32,
        ) as *mut u16;

        vramSetBankF(VRAM_F_MAIN_BG as u32);
    }
}

#[no_mangle]
extern "C" fn main() -> c_int {
    unsafe {
        consoleDemoInit();
        println!("Loading Dog Logo...");

        let bg_id = bgInit(3, BgType_Bmp8 as u32, BgSize_B8_256x256 as u32, 0, 0);

        let init_scale_x: s32 = f32_to_s32(256.0 / 192.0);
        let init_scale_y: s32 = f32_to_s32(256.0 / 192.0);
        let init_scroll_x: s32 = (256.0 as s32 - init_scale_x) / 2 as s32;

        println!(
            "Initial Scale X: {}, Scale Y: {}, Scroll X: {}",
            init_scale_x, init_scale_y, init_scroll_x as s32
        );

        bgSetScale(bg_id, init_scale_x, init_scale_y);
        bgSetScroll(bg_id, init_scroll_x, 0);
        bgUpdate();

        let bg_gfx_ptr = bgGetGfxPtr(bg_id) as *mut u16;

        dmaCopy(
            DOG_LOGO_BITMAP.get_c_data(),
            bg_gfx_ptr as *mut c_void,
            65536,
        );
        dmaCopy(
            DOG_LOGO_PALETTE.get_c_data(),
            BG_PALETTE as *mut c_void,
            512,
        );

        let mut scroll_y: i32 = 0;
        let mut scroll_x: i32 = init_scroll_x as i32;
        let mut show: bool = true;

        loop {
            scanKeys();

            let keys_pressed = keysDown();
            let keys_held = keysHeld();

            let old_scroll_x = init_scroll_x;
            let old_scroll_y = 0;

            match keys_pressed {
                KEY_SELECT => {
                    scroll_x = init_scroll_x;
                    scroll_y = 0;
                    bgSetScroll(bg_id, scroll_x, scroll_y);
                    bgUpdate();
                }
                KEY_START => {
                    println!("Start pressed, exiting...");
                    swiWaitForVBlank();
                    break;
                }
                KEY_A => {
                    show = !show;
                    if show {
                        bgShow(bg_id);
                    } else {
                        bgHide(bg_id);
                    }

                    println!("Show/Hide toggled: {}", show);
                }
                _ => {}
            }

            if keys_held & KEY_UP != 0 {
                scroll_y = (scroll_y + 1).max(-255);
            }

            if keys_held & KEY_DOWN != 0 {
                scroll_y = (scroll_y - 1).min(255);
            }

            if keys_held & KEY_LEFT != 0 {
                scroll_x = (scroll_x + 1).max(-255);
            }

            if keys_held & KEY_RIGHT != 0 {
                scroll_x = (scroll_x - 1).min(255);
            }

            if old_scroll_y != scroll_y || old_scroll_x != scroll_x {
                println!("Scroll changed to: {}/{}", scroll_y, scroll_x);
                bgSetScroll(bg_id, scroll_x, scroll_y);
                bgUpdate();
            }

            swiWaitForVBlank();
        }

        println!("Exiting program...");
        swiWaitForVBlank();
    }

    return 0;
}
