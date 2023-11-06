

use super::{
    font_character::{FontCharacter, CHAR_BYTE_SIZE, EMPTY_CHARACTER, CHAR_BIT_WIDTH}, TEXT_MARGIN,
};

pub const TOTAL_CHARS: usize = 255;
const DEFAULT_FONT: (&'static [u8], &str) = (include_bytes!("../../../fonts/font.dat"), "default");

// array of all read fonts
static mut FONTS: Vec<FontData> = vec![];

pub struct FontData {
    name: String,
    pub characters: [FontCharacter; TOTAL_CHARS],
}

impl FontData {
    fn new(characters_bytes: &[u8], name: String) -> Self {
        // create empty characters list
        let mut characters: [FontCharacter; TOTAL_CHARS] = [EMPTY_CHARACTER; TOTAL_CHARS];

        // load all characters from bytes
        for (i, ch) in characters.iter_mut().enumerate() {
            // get bits for a character
            let mut bytes = [0u8; CHAR_BYTE_SIZE];
            bytes.copy_from_slice(
                &characters_bytes[(CHAR_BYTE_SIZE * (i))..(CHAR_BYTE_SIZE * (i + 1))],
            );
            ch.set_bytes(bytes);
        }
        Self { name, characters }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Font(usize);

impl Font {
    pub fn new<'a>(characters_bytes: &[u8], name: String) -> Self {
        Self::load_font(characters_bytes, name)
    }
    pub fn text_width(&self, text: &String, size: u32) -> u32 {
        let font_data = self.get_font_data();
        let mut w = 0;
        for ch in text.chars() {
            let charcode = ch as usize;
            w += (((font_data.characters[charcode].width() + TEXT_MARGIN) as f32*(size) as f32/CHAR_BIT_WIDTH as f32)) as u32;
        }
        return w + TEXT_MARGIN;
    }
    pub fn text_height(&self, _text: &String, size: u32) -> u32 {
        return size;
    }

    // load a font, creates it if first load
    fn load_font(characters_bytes: &[u8], name: String) -> Self {
        // search for the font
        let font_list = unsafe { &mut FONTS };
        for (i, font_data) in font_list.into_iter().enumerate() {
            if font_data.name == name {
                return Font(i);
            }
        }

        // font not found => add new font
        let font = FontData::new(characters_bytes, name);
        font_list.push(font);
        return Font(font_list.len() - 1);
    }

    pub fn get_font_data(&self) -> &FontData {
        let font_list = unsafe { &FONTS };
        let data = font_list.get(self.0).unwrap();
        return data;
    }
}

impl Default for Font{
    fn default() -> Self {
        Self::new(DEFAULT_FONT.0, DEFAULT_FONT.1.to_string())
    }
}