pub mod format;

use std::cell::RefCell;

use super::color;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<color::ARGB>
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![color::ARGB::default(); width * height]
        }
    }

    pub fn new_pixels(width: usize, height: usize, pixels: Vec<color::ARGB>) -> Image {
        Image {
            width,
            height,
            pixels
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row(&self, j: usize) -> &[color::ARGB] {
        &self.pixels[(self.calculate_index(0, j))..(self.calculate_index(self.width, j))]
    }

    pub fn iter(&self) -> ImageIterator {
        ImageIterator::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageIterator<'a> {
    row: RefCell<usize>,
    image: &'a Image
}

impl<'a> ImageIterator<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self {
            row: RefCell::from(0),
            image
        }
    }

    fn row(&self) -> usize {
        *self.row.borrow()
    }

    fn increment(&self) -> usize {
       let mut row = self.row.borrow_mut();
       let current = *row;
       *row += 1;
       current
    }
}

impl<'a> Iterator for &'a ImageIterator<'a> {
    type Item = &'a [color::ARGB];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row() >= self.image.height() {
            None
        }
        else {
            Some(self.image.row(self.increment()))
        }
    }
}