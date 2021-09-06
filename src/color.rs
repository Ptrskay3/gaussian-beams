use std::ops::{Add, AddAssign, Div};

#[derive(Copy, Clone, Debug)]
pub struct CColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl CColor {
    pub fn new(&self, r: u8, g: u8, b: u8, a: u8) -> CColor {
        CColor { r, g, b, a }
    }

    pub fn as_sl(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Add<f64> for CColor {
    type Output = CColor;
    fn add(self, other: f64) -> CColor {
        CColor {
            r: self.r + (other * 255.0f64) as u8,
            g: self.g + (other * 255.0f64) as u8,
            b: self.b + (other * 255.0f64) as u8,
            a: self.a,
        }
    }
}

impl Div<u32> for CColor {
    type Output = CColor;
    fn div(self, other: u32) -> CColor {
        CColor {
            r: self.r / other as u8,
            g: self.g / other as u8,
            b: self.b / other as u8,
            a: self.a,
        }
    }
}

impl AddAssign<f64> for CColor {
    fn add_assign(&mut self, other: f64) {
        self.r = self.r + (other * 255.0f64) as u8;
        self.g = self.g + (other * 255.0f64) as u8;
        self.b = self.b + (other * 255.0f64) as u8;
    }
}
