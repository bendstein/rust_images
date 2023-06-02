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

    pub fn length(&self) -> usize {
        self.width() * self.height()
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

    fn move_by(&self, n: isize) -> usize {
        let mut row = self.row.borrow_mut();
        let current = *row;

        //Add n to row, bounding result between [0, number of rows]
        *row = row.checked_add_signed(n)
            .unwrap_or_else(|| if n <= 0_isize { 0_usize } else { self.image.height() })
            .min(self.image.height());
        current
    }

}

impl<'a> Iterator for &'a ImageIterator<'a> {
    type Item = &'a [color::ARGB];

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(1)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.row() + n > self.image.height() {
            None
        }
        else {
            Some(self.image.row(self.move_by(n as isize)))
        }
    }
}

impl<'a> DoubleEndedIterator for &'a ImageIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nth_back(1)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.row() + n > self.image.height() {
            None
        }
        else {
            Some(self.image.row(self.image.height() - self.move_by(n as isize) - 1))
        }
    }
}