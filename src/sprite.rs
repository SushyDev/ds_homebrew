use core::ffi::*;
use libnds_sys::arm9_bindings::*;
use libnds_sys::video_registers::*;

// Sprite structure to hold sprite data and state
pub struct Sprite {
    gfx_ptr: *mut u16,
    affine_id: i32,
    angle_degrees: i32,
    x: i32,
    y: i32,
    sprite_id: i32,
    size: u32,
    color_format: u32,
}

impl Sprite {
    /// Create a new sprite with the given parameters
    pub fn new(
        sprite_id: i32,
        x: i32,
        y: i32,
        size: u32,
        color_format: u32,
        graphics_data: *const c_void,
        graphics_size: u32,
    ) -> Self {
        unsafe {
            let gfx_ptr = oamAllocateGfx(&raw mut oamMain, size, color_format) as *mut u16;

            // Load sprite graphics
            dmaCopy(graphics_data, gfx_ptr as *mut c_void, graphics_size);

            let sprite = Self {
                gfx_ptr,
                affine_id: sprite_id, // Use sprite_id as affine_id for simplicity
                angle_degrees: 0,
                x,
                y,
                sprite_id,
                size,
                color_format,
            };

            // Initial display
            sprite.update_display();
            sprite
        }
    }

    /// Rotate the sprite by a given number of degrees (relative rotation)
    pub fn rotate(&mut self, degrees: i32) {
        self.angle_degrees = (self.angle_degrees + degrees + 360) % 360;
        self.update_display();
    }

    /// Set the sprite's absolute rotation in degrees
    pub fn set_rotation(&mut self, degrees: i32) {
        self.angle_degrees = (degrees + 360) % 360;
        self.update_display();
    }

    /// Reset rotation to 0 degrees
    pub fn reset_rotation(&mut self) {
        self.angle_degrees = 0;
        self.update_display();
    }

    /// Move the sprite to a new position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.update_display();
    }

    /// Get current position as (x, y) tuple
    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Get current rotation in degrees
    pub fn get_rotation(&self) -> i32 {
        self.angle_degrees
    }

    /// Internal function to update the sprite display
    fn update_display(&self) {
        unsafe {
            if self.angle_degrees == 0 {
                // No rotation - don't use affine transformation
                oamSet(
                    &raw mut oamMain,
                    self.sprite_id,
                    self.x,
                    self.y,
                    0, // priority
                    0, // palette
                    self.size,
                    self.color_format,
                    self.gfx_ptr as *mut c_void,
                    -1,    // No affine transformation
                    false, // double size
                    false, // hide
                    false, // h flip
                    false, // v flip
                    false, // mosaic
                );
            } else {
                // Convert degrees to DS units (32768 units = 360 degrees)
                // Negative for clockwise rotation
                let angle_ds_units = -(self.angle_degrees * 32768 / 360);

                // Set up rotation matrix
                oamRotateScale(
                    &raw mut oamMain,
                    self.affine_id,
                    angle_ds_units,
                    256, // Normal scale X
                    256, // Normal scale Y
                );

                // Apply affine transformation to sprite
                oamSet(
                   &raw mut oamMain,
                    self.sprite_id,
                    self.x,
                    self.y,
                    0, // priority
                    0, // palette
                    self.size,
                    self.color_format,
                    self.gfx_ptr as *mut c_void,
                    self.affine_id, // Use affine transformation
                    false,          // double size
                    false,          // hide
                    false,          // h flip
                    false,          // v flip
                    false,          // mosaic
                );
            }
        }
    }
}

/// Initialize the sprite system
pub fn init_sprite_system() {
    unsafe {
        vramSetBankA(VRAM_A_MAIN_SPRITE as u32);
        oamInit(&raw mut oamMain, SpriteMapping_1D_256, false);
    }
}

/// Update the OAM (call this every frame)
pub fn update_sprites() {
    unsafe {
        oamUpdate(&raw mut oamMain);
    }
}

/// Load a palette into sprite memory
pub fn load_sprite_palette(palette_data: *const c_void, size: u32) {
    unsafe {
        dmaCopy(palette_data, SPRITE_PALETTE as *mut c_void, size);
    }
}
