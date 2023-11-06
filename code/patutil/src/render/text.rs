pub mod font;
pub mod font_character;

use self::font::{Font};

use crate::{Size, Color};

pub const TEXT_MARGIN: u32 = 3;

pub struct Text {
    text: String,
    size: u32,
    color: Color,
    font: Font,
}
impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Text {
            text: text.into(),
            size: 15,
            color: [255,255,255,255].into(),
            font: Font::default()
        }
    }
    pub fn get(&self) -> &String {
        &self.text
    }
    pub fn size(&self) -> Size {
        [
            self.font.text_width(&self.text, self.size),
            self.font.text_height(&self.text, self.size),
        ].into()
    }
    pub fn char_size(&self) -> u32{
        return self.size;
    }
    pub fn font(&self) -> Font {
        self.font
    }
    pub fn color(&self) -> &Color{
        &self.color
    }
    pub fn set_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
    pub fn set_color(mut self, color: impl Into<Color>) -> Self{
        self.color = color.into();
        self
    }

    pub fn width(&self) -> u32 {
        return self.font.text_width(&self.text, self.size);
    }
    pub fn height(&self) -> u32 {
        return self.font.text_height(&self.text, self.size);
    }

    pub fn set_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl<T: Into<String>> From<T> for Text {
    fn from(text: T) -> Self {
        Self::new(text.into())
    }
}
