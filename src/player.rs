#![no_std]

use crate::sprite::Sprite;

pub struct Player {
    y: f32,
    velocity: f32,
    sprite: Sprite,
}

impl Player {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            y: 0.0,
            velocity: 0.0,
            sprite,
        }
    }

    pub fn update(&mut self, space_pressed: bool) {
        if space_pressed {
            self.velocity = -8.0; // Jump velocity
        }

        self.velocity += 0.5; // Gravity effect
        
        self.y += self.velocity;

        if self.y < 0.0 {
            self.y = 0.0; // Prevent going above the top
        } else if self.y > 192.0 - 16.0 {
            self.y = 192.0 - 16.0; // Prevent going below the bottom
        }

        let (sprite_x, _) = self.sprite.get_position();

        self.sprite.set_position(sprite_x, self.y as i32);
    }
}

