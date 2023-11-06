use std::{fmt::Display, str::FromStr};

use patfile::{pscan, pwrite};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_float(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.,
            self.g as f32 / 255.,
            self.b as f32 / 255.,
            self.a as f32 / 255.,
        ]
    }
}

impl From<[u8; 4]> for Color {
    fn from(data: [u8; 4]) -> Self {
        Self {
            r: data[0],
            g: data[1],
            b: data[2],
            a: data[3],
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut a: u8 = 0;

        let mut it = s.bytes().into_iter();
        pscan!(&mut it => "[{},{},{},{}]", r,g,b,a ).unwrap();

        Ok(Color::new(r, g, b, a))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pwrite!("[{},{},{},{}]" => f, self.r, self.g, self.b, self.a).unwrap();
        Ok(())
    }
}
