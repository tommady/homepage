use rand::Rng;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Octocat {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl Octocat {
    pub fn new() -> Octocat {
        let mut octocat = Vec::new();
        for line in include_str!("../textures/octocat.asciiart").lines() {
            let row: Vec<char> = line.chars().collect();
            octocat.push(row);
        }

        let height = octocat.len();
        let width = octocat[0].len();

        Octocat {
            width: width,
            height: height,
            original: octocat,
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

#[wasm_bindgen]
pub struct Email {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl Email {
    pub fn new() -> Email {
        let mut email = Vec::new();
        for line in include_str!("../textures/email.asciiart").lines() {
            let row: Vec<char> = line.chars().collect();
            email.push(row);
        }

        let height = email.len();
        let width = email[0].len();

        Email {
            width: width,
            height: height,
            original: email,
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

#[wasm_bindgen]
pub struct Linkedin {
    width: usize,
    height: usize,
    original: Vec<Vec<char>>,
    output: Vec<u8>,
    rng: rand::rngs::ThreadRng,
}

#[wasm_bindgen]
impl Linkedin {
    pub fn new() -> Linkedin {
        let mut linkedin = Vec::new();
        for line in include_str!("../textures/linkedin.asciiart").lines() {
            let row: Vec<char> = line.chars().collect();
            linkedin.push(row);
        }

        let height = linkedin.len();
        let width = linkedin[0].len();

        Linkedin {
            width: width,
            height: height,
            original: linkedin,
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
