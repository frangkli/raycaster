// Don't link the Rust standard library
#![no_std]

use core::{arch::wasm32, panic::PanicInfo};
use libm::{cosf, sinf};

// Import WASM functions
extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

// Pointer to keyboard state
const GAMEPAD1: *const u8 = 0x16 as *const u8;

// Binary masks for GAMEPAD
const BUTTON_LEFT: u8 = 16;   // 0b00010000
const BUTTON_RIGHT: u8 = 32;  // 0b00100000
const BUTTON_UP: u8 = 64;     // 0b01000000
const BUTTON_DOWN: u8 = 128;  // 0b10000000

// Map walls
const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

const STEP_SIZE: f32 = 0.045;

// Game State
struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

impl State {
    // Move the character
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        // Store current position just in case
        let previous_position = (self.player_x, self.player_y);

        // Move the player
        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE;
        }
        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }
        if right {
            self.player_angle -= STEP_SIZE;
        }
        if left {
            self.player_angle += STEP_SIZE;
        }

        // If moving into a wall, undo the move
        if point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }
}

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

// Required by #![no_std] to handle panic
#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

// Check if the map contains a wall at a point
fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true,
    }
}

#[no_mangle]
unsafe fn update() {
    STATE.update(
        *GAMEPAD1 & BUTTON_UP != 0,
        *GAMEPAD1 & BUTTON_DOWN != 0,
        *GAMEPAD1 & BUTTON_LEFT != 0,
        *GAMEPAD1 & BUTTON_RIGHT != 0,
    );
}