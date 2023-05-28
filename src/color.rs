#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct XYZA {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LABA {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HSVA {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub alpha: u8
}