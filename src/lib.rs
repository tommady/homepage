use rand::Rng;
use wasm_bindgen::prelude::*;

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
pub struct Octocat {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
}

#[wasm_bindgen]
impl Octocat {
    pub fn new() -> Octocat {
        let mut octocat = Vec::new();
        for line in include_str!("../textures/octocat.asciiart").lines() {
            let row: Vec<char> = line.chars().rev().collect();
            octocat.push(row);
        }

        let width = octocat[0].len();
        let height = octocat.len();

        Octocat {
            width: width,
            height: height,
            original: octocat,
            // muliple 4 is for rgba color
            output: vec![0; width * height * 4],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_output_ptr(&self) -> *const u8 {
        self.output.as_ptr()
    }

    pub fn gen(&mut self) {
        let mut rng = rand::thread_rng();
        let red: u8 = rng.gen_range(0, 255);
        let green: u8 = rng.gen_range(0, 255);
        let blue: u8 = rng.gen_range(0, 255);

        for y in 0..self.height {
            // log!("y:{}", y);
            for x in 0..self.width {
                // log!("x:{}", x);
                if self.original[y][x] != ' ' {
                    let index: usize = y * self.width + x;
                    let rgba_index: usize = index * 4;

                    self.output[rgba_index + 0] = red;
                    self.output[rgba_index + 1] = green;
                    self.output[rgba_index + 2] = blue;
                    self.output[rgba_index + 3] = 255;
                }
            }
        }
    }
}
