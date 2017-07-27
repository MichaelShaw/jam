use image::Rgba;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

use std::ops::Mul;


impl Mul<f32> for Color {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let nr = (self.r as f32) * rhs;
        let ng = (self.g as f32) * rhs;
        let nb = (self.b as f32) * rhs;
        let na = (self.a as f32) * rhs;

        Color {
            r: nr as u8,
            g: ng as u8,
            b: nb as u8,
            a: na as u8,
        }
    }
}


pub const RED: Color = Color { r:255, g:0, b:0, a:255 };
pub const GREEN: Color = Color { r:0, g:255, b:0, a:255 };
pub const YELLOW: Color = Color { r:255, g:255, b:0, a:255 };
pub const BLUE: Color = Color { r:0, g:0, b:255, a:255 };
pub const WHITE: Color = Color { r:255, g:255, b:255, a:255 };
pub const GRAY: Color = Color { r:125, g:125, b:125, a:255 };
pub const BLACK: Color = Color { r:0, g:0, b:0, a:255 };
pub const PINK: Color = Color { r:255, g:105, b:180, a:255 };

pub const ALL: [Color; 8] = [RED, GREEN, YELLOW, BLUE, WHITE, GRAY, BLACK, PINK];

pub type ColorRaw = [u8; 4];
pub type ColorFloatRaw = [f32; 4];

pub fn rgba(r:u8, g:u8, b:u8, a: u8) -> Color {
    Color { r:r, g:g, b:b, a:a }
}

pub fn rgb(r:u8, g:u8, b:u8) -> Color {
    Color { r:r, g:g, b:b, a: 255}
}

pub fn as_rgba8(color:Color) -> Rgba<u8> {
    Rgba { data: color.raw() }
}

impl Color {
    pub fn with_alpha(&self, new_alpha: f32) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (new_alpha * 255.0) as u8,
        }
    }

    pub fn rf(&self) -> f32 {
        (self.r as f32) / 255.0
    }

    pub fn gf(&self) -> f32 {
        (self.g as f32) / 255.0
    }

    pub fn bf(&self) -> f32 {
        (self.b as f32) / 255.0
    }

    pub fn af(&self) -> f32 {
        (self.a as f32) / 255.0
    }

    pub fn raw(&self) -> ColorRaw {
        [self.r, self.g, self.b, self.a]
    }

    pub fn float_raw(&self) -> ColorFloatRaw {
        [self.rf(), self.gf(), self.bf(), self.af()]
    }

    pub fn tup(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn float_tup(&self) -> (f32, f32, f32, f32) {
        (self.rf(), self.gf(), self.bf(), self.af())
    }
}