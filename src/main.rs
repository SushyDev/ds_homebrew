#![no_main]
#![no_std]

extern crate alloc;
use core::ffi::*;
use libnds_sys::arm9_bindings::*;
use libnds_sys::println;

mod assets;
use assets::*;

mod sprite;
use sprite::*;

mod player;
use player::Player;

#[no_mangle]
extern "C" fn main() -> c_int {
    unsafe {
        videoSetMode(MODE_5_2D as u32);
        consoleDemoInit();
        println!("Loading Sprite...");

        // Initialize sprite system
        init_sprite_system();
        
        println!("Sprite Bitmap Length: {}", SPRITE_BIRD_BITMAP.data.len());
        println!("Sprite Palette Length: {}", SPRITE_BIRD_PALETTE.data.len());

        load_sprite_palette(SPRITE_BIRD_PALETTE.get_data_c(), 512);
        
        let bird_sprite = Sprite::new(
            0, // sprite ID
            50, 50, // position
            SpriteSize_16x16 as u32,
            SpriteColorFormat_256Color as u32,
            SPRITE_BIRD_BITMAP.get_data_c(),
            256, // graphics size
        );
        
        let mut player = Player::new(bird_sprite);

        loop {
            scanKeys();
            let keys_pressed = keysDown();

            match keys_pressed {
                KEY_SELECT => {
                }
                KEY_START => {
                    println!("Start pressed, exiting...");
                    swiWaitForVBlank();
                    break;
                }
                KEY_RIGHT => {
                }
                KEY_LEFT => {
                }
                KEY_UP => {
                }
                KEY_DOWN => {
                }
                KEY_B => {
                }
                _ => {}
            }

            if keys_pressed & KEY_A != 0 {
                player.update(true);
            } else {
                player.update(false);
            }

            swiWaitForVBlank();
            update_sprites();
        }

        println!("Exiting program...");
        swiWaitForVBlank();
    }

    return 0;
}
