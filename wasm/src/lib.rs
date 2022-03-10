use rand::{rngs::SmallRng, Rng, SeedableRng};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Animation {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
    rng: SmallRng,
}

// since this issue
// https://github.com/rustwasm/wasm-bindgen/issues/2774
// so do a workaround here
#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Animation {
    pub fn new(name: &str) -> Animation {
        let mut animation = Vec::new();

        let ascii = match name {
            "octocat" => include_str!("../textures/octocat.asciiart"),
            "email" => include_str!("../textures/email.asciiart"),
            "linkedin" => include_str!("../textures/linkedin.asciiart"),
            "blog" => include_str!("../textures/blog.asciiart"),
            _ => include_str!("../textures/octocat.asciiart"),
        };

        for line in ascii.lines() {
            let row: Vec<char> = line.chars().collect();
            animation.push(row);
        }

        let height = animation.len();
        let width = animation[0].len();
        let seed = match name {
            "octocat" => 3345678,
            "email" => 449,
            "linkedin" => 666,
            "blog" => 999,
            _ => 3345678,
        };

        Animation {
            width,
            height,
            original: animation,
            // muliple 4 is for rgba color
            output: vec![0; width * height * 4],
            rng: SmallRng::seed_from_u64(seed),
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
        let red: u8 = self.rng.gen_range(0..255);
        let green: u8 = self.rng.gen_range(0..255);
        let blue: u8 = self.rng.gen_range(0..255);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.original[y][x] != ' ' {
                    let index: usize = y * self.width + x;
                    let rgba_index: usize = index * 4;

                    self.output[rgba_index] = red;
                    self.output[rgba_index + 1] = green;
                    self.output[rgba_index + 2] = blue;
                    self.output[rgba_index + 3] = 255;
                }
            }
        }
    }
}
