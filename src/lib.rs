use rand::Rng;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const OCTOCAT_WIDTH: usize = 49;
const OCTOCAT_HEIGHT: usize = 22;
const OCTOCAT_OUTPUT_SIZE: usize = OCTOCAT_WIDTH * OCTOCAT_HEIGHT * 4;
static mut OCTOCAT_OUTPUT_BUFFER: [u8; OCTOCAT_OUTPUT_SIZE] = [0; OCTOCAT_OUTPUT_SIZE];

#[wasm_bindgen]
pub fn get_octocat_output_buffer_ptr() -> *const u8 {
    let ptr: *const u8;
    unsafe {
        ptr = OCTOCAT_OUTPUT_BUFFER.as_ptr();
    }
    return ptr;
}

#[wasm_bindgen]
pub fn gen_octocat() {
    let mut rng = rand::thread_rng();
    let red: u8 = rng.gen_range(0, 255);
    let green: u8 = rng.gen_range(0, 255);
    let blue: u8 = rng.gen_range(0, 255);

    let mut octocat = Vec::new();
    for line in include_str!("../textures/octocat.asciiart").lines() {
        let row: Vec<char> = line.chars().rev().collect();
        octocat.push(row);
    }

    for y in 0..OCTOCAT_HEIGHT {
        log!("y:{}", y);
        for x in 0..OCTOCAT_WIDTH {
            log!("x:{}", x);
            if octocat[y][x] != ' ' {
                // log!("x:{}, y:{}, {}", x, y, octocat[y][x]);

                let index: usize = y * OCTOCAT_WIDTH + x;
                let rgba_index: usize = index * 4;

                unsafe {
                    OCTOCAT_OUTPUT_BUFFER[rgba_index + 0] = red;
                    OCTOCAT_OUTPUT_BUFFER[rgba_index + 1] = green;
                    OCTOCAT_OUTPUT_BUFFER[rgba_index + 2] = blue;
                    OCTOCAT_OUTPUT_BUFFER[rgba_index + 3] = 255;
                }
            }
        }
    }
}
