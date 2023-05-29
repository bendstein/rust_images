pub mod conversion;

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

fn distance_euclidean(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    f32::sqrt(
        (a.0 - b.0).powi(2)
        + (a.1 - b.1).powi(2)
        + (a.2 - b.2).powi(2)
    )
}

fn distance_manhattan(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    (a.0 - b.0).abs()
    + (a.1 - b.1).abs()
    + (a.2 - b.2).abs()
}

fn as_u32(v: (u8, u8, u8, u8), little_endian: bool) -> u32 {
    if little_endian {
        (v.0 as u32)
        + ((v.1 as u32) << 8)
        + ((v.2 as u32) << 16)
        + ((v.3 as u32) << 24)
    }
    else {
        ((v.0 as u32) << 24)
        + ((v.1 as u32) << 16)
        + ((v.2 as u32) << 8)
        + (v.3 as u32)
    }
}

impl RGBA {
    pub fn distance_euclidean(&self, other: &Self) -> f32 {
        distance_euclidean(
            (self.red as f32, self.blue as f32, self.green as f32), 
            (other.red as f32, other.blue as f32, other.green as f32)
        )
    }

    pub fn distance_manhattan(&self, other: &Self) -> f32 {
        distance_manhattan(
            (self.red as f32, self.blue as f32, self.green as f32), 
            (other.red as f32, other.blue as f32, other.green as f32)
        )
    }

    pub fn as_u32(&self, little_endian: bool) -> u32 {
        as_u32((self.alpha, self.red, self.green, self.blue), little_endian)
    }
}

impl XYZA {
    pub fn distance_euclidean(&self, other: &Self) -> f32 {
        distance_euclidean(
            (self.x, self.y, self.z), 
            (other.x, other.y, other.z)
        )
    }

    pub fn distance_manhattan(&self, other: &Self) -> f32 {
        distance_manhattan(
            (self.x, self.y, self.z), 
            (other.x, other.y, other.z)
        )
    }
}

impl LABA {
    pub fn distance_euclidean(&self, other: &Self) -> f32 {
        distance_euclidean(
            (self.l, self.a, self.b), 
            (other.l, other.a, other.b)
        )
    }

    pub fn distance_manhattan(&self, other: &Self) -> f32 {
        distance_manhattan(
            (self.l, self.a, self.b), 
            (other.l, other.a, other.b)
        )
    }
}

impl HSVA {
    pub fn distance_euclidean(&self, other: &Self) -> f32 {
        distance_euclidean(
            (self.h, self.s, self.v), 
            (other.h, other.s, other.v)
        )
    }

    pub fn distance_manhattan(&self, other: &Self) -> f32 {
        distance_manhattan(
            (self.h, self.s, self.v), 
            (other.h, other.s, other.v)
        )
    }
}