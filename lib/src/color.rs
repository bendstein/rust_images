pub mod conversion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ARGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AXYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ALAB {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AHSV {
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

fn from_u32(n: u32, little_endian: bool) -> (u8, u8, u8, u8) {
    if little_endian {
        (
            (n & 0xFF) as u8,
            ((n >> 8) & 0xFF) as u8,
            ((n >> 16) & 0xFF) as u8,
            ((n >> 24) & 0xFF) as u8
        )
    }
    else {
        (
            ((n >> 24) & 0xFF) as u8,
            ((n >> 16) & 0xFF) as u8,
            ((n >> 8) & 0xFF) as u8,
            (n & 0xFF) as u8
        )
    }
}

impl ARGB {
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

    pub fn from_u32(n: u32, little_endian: bool) -> Self {
        let v = from_u32(n, little_endian);
        Self {
            alpha: v.0,
            red: v.1,
            green: v.2,
            blue: v.3
        }
    }

    pub fn with_alpha(&self, alpha: u8) -> Self {
        Self {
            alpha,
            red: self.red,
            green: self.green,
            blue: self.blue
        }
    }

    pub fn with_red(&self, red: u8) -> Self {
        Self {
            alpha: self.alpha,
            red,
            green: self.green,
            blue: self.blue
        }
    }

    pub fn with_green(&self, green: u8) -> Self {
        Self {
            alpha: self.alpha,
            red: self.red,
            green,
            blue: self.blue
        }
    }

    pub fn with_blue(&self, blue: u8) -> Self {
        Self {
            alpha: self.alpha,
            red: self.red,
            green: self.green,
            blue
        }
    }
}

impl From<u32> for ARGB {
    fn from(value: u32) -> Self {
        Self::from_u32(value, false)
    }
}

impl From<ARGB> for u32 {
    fn from(value: ARGB) -> Self {
        value.as_u32(false)
    }
}

impl AXYZ {
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

impl ALAB {
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

impl AHSV {
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