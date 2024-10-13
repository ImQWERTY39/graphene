#![allow(dead_code)]

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 800;
pub const CENTER_WIDTH: i32 = WINDOW_WIDTH as i32 / 2;
pub const CENTER_HEIGHT: i32 = WINDOW_HEIGHT as i32 / 2;

pub const X_MAX: f64 = 12.0;
pub const Y_MAX: f64 = 8.0;

const HORIZONTAL_SCALE: f64 = X_MAX / CENTER_WIDTH as f64;
const VERTICAL_SCALE: f64 = Y_MAX / CENTER_HEIGHT as f64;

// Screen to custom: screen coords -> shifted origin coords - scale factor > custom coords
// custom to Screen: custom coords - 1/scale factor > shifted origin coords -> screen coords

pub fn posx_from_screenx(x: i32) -> f64 {
    (x - CENTER_WIDTH) as f64 * HORIZONTAL_SCALE
}

pub fn posy_from_screeny(y: i32) -> f64 {
    (CENTER_HEIGHT - y) as f64 * VERTICAL_SCALE
}

pub fn screenx_from_posx(x: f64) -> i32 {
    (x / HORIZONTAL_SCALE) as i32 + CENTER_WIDTH
}

pub fn screeny_from_posy(y: f64) -> i32 {
    CENTER_HEIGHT - (y / VERTICAL_SCALE) as i32
}
