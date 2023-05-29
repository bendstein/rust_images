pub mod format;

use super::color;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<color::ARGB>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let len = width * height;
        Image {
            width,
            height,
            pixels: Vec::with_capacity(len)
        }
    }

    fn calculate_index(&self, i: usize, j: usize) -> usize {
        self.width * j + i
    }

    pub fn get(&self, i: usize, j: usize) -> Option<color::ARGB> {
        let index = self.calculate_index(i, j);

        if index > self.pixels.capacity() {
            None
        }
        else {
            Some(self.pixels[index])
        }
    }

    pub fn set(&mut self, value: color::ARGB, i: usize, j: usize) {
        let index = self.calculate_index(i, j);
        self.pixels[index] = value;
    }
}