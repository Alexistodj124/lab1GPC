use crate::bmp::write_bmp_file;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn get_current_color(&self) -> u32 {
        self.current_color
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn render_buffer(&self, filename: &str) -> std::io::Result<()> {
        write_bmp_file(filename, &self.buffer, self.width, self.height)
    }
    pub fn flip_horizontal(&mut self) {
        for y in 0..self.height {
            for x in 0..(self.width / 2) {
                let left_index = y * self.width + x;
                let right_index = y * self.width + (self.width - 1 - x);
                self.buffer.swap(left_index, right_index);
            }
        }
    }

    pub fn flip_vertical(&mut self) {
        for y in 0..(self.height / 2) {
            for x in 0..self.width {
                let top_index = y * self.width + x;
                let bottom_index = (self.height - 1 - y) * self.width + x;
                self.buffer.swap(top_index, bottom_index);
            }
        }
    }
}
