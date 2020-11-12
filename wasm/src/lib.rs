use rand::Rng;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Animation {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl Animation {
    pub fn new(name: &str) -> Animation {
        let mut animation = Vec::new();

        // this is weird...
        // if not using the #[allow(unused_assignments)], will showing:
        // unused_assignments: value assigned to `ascii` is never read  note: `#[warn(unused_assignments)]` on by default help:
        // maybe it is overwritten before being read?
        #[allow(unused_assignments)]
        let mut ascii: &str = "";

        match name {
            "octocat" => ascii = include_str!("../textures/octocat.asciiart"),
            "email" => ascii = include_str!("../textures/email.asciiart"),
            "linkedin" => ascii = include_str!("../textures/linkedin.asciiart"),
            _ => ascii = include_str!("../textures/octocat.asciiart"),
        }

        for line in ascii.lines() {
            let row: Vec<char> = line.chars().collect();
            animation.push(row);
        }

        let height = animation.len();
        let width = animation[0].len();

        Animation {
            width: width,
            height: height,
            original: animation,
            // muliple 4 is for rgba color
            output: vec![0; width * height * 4],
            rng: rand::thread_rng(),
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
        let red: u8 = self.rng.gen_range(0, 255);
        let green: u8 = self.rng.gen_range(0, 255);
        let blue: u8 = self.rng.gen_range(0, 255);

        for y in 0..self.height {
            for x in 0..self.width {
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
