pub mod format;

use super::color;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<color::RGBA>
}