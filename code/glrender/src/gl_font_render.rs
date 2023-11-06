use std::ffi::c_void;

use patutil::{
    text::{
        font::{Font, TOTAL_CHARS},
        font_character::{CHAR_BIT_SIZE, CHAR_BIT_WIDTH, CHAR_BIT_HEIGHT},
        Text, TEXT_MARGIN,
    },
    Pos, Size,
};

use super::{ShaderProgram, VertexBuffer};

static mut FONT_TEXTURES: Vec<FontTextures> = vec![];

const DEFAULT_CHARTEX: CharTex = CharTex { id: 0, width: 0 };

struct FontTextures {
    font: Font,
    textures: [CharTex; TOTAL_CHARS],
}

struct CharTex {
    id: u32,
    width: u32,
}

pub struct GLFontRender {
    shader_program: ShaderProgram,
    vbo: VertexBuffer,
}
impl GLFontRender {
    pub fn new() -> Self {
        let vbo = VertexBuffer::from_rect().unwrap();
        let v_source = include_str!("shaders/text_shader.vert");
        let f_source = include_str!("shaders/text_shader.frag");
        let shader_program = ShaderProgram::new(v_source, f_source).unwrap();
        Self {
            shader_program,
            vbo,
        }
    }

    fn load_textures(&self, font: Font) -> &FontTextures {
        // search for the textures
        {
            let ft_list = unsafe { &mut FONT_TEXTURES };
            for ft in ft_list.iter() {
                if ft.font == font {
                    return ft;
                }
            }
        }

        // font textures not found => create new textures
        let ft = Self::create_textures(font);
        unsafe { FONT_TEXTURES.push(ft) };
        return unsafe { FONT_TEXTURES.last().unwrap() };
    }

    fn create_textures(font: Font) -> FontTextures {
        let mut ft: FontTextures = FontTextures {
            font,
            textures: [DEFAULT_CHARTEX; TOTAL_CHARS],
        };

        // create texture data
        let mut texture_data = [[100u8; CHAR_BIT_SIZE]; TOTAL_CHARS];
        let fontdata = ft.font.get_font_data();

        for (i, ch) in fontdata.characters.iter().enumerate() {
            for bit_x in 0..CHAR_BIT_WIDTH{
                for bit_y in 0..CHAR_BIT_HEIGHT{
                    texture_data[i][bit_y*CHAR_BIT_HEIGHT + bit_x] = 255u8* ch.get_bit(bit_x, bit_y) as u8;
                }
            }
        }


        for (i, tex) in ft.textures.iter_mut().enumerate() {
            unsafe {
                gl::GenTextures(1, &mut tex.id as *mut u32);
                gl::BindTexture(gl::TEXTURE_2D, tex.id);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    CHAR_BIT_WIDTH as i32,
                    CHAR_BIT_WIDTH as i32,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    texture_data[i].as_ptr() as *const c_void,
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            tex.width = fontdata.characters[i].width();
        }

        return ft;
    }

    pub fn draw_text(&self, screen_size: Size, pos: Pos, text: &Text) {
        // get font textures
        let ft = self.load_textures(text.font());

        self.shader_program.set_used();
        self.shader_program.bind_uniform("uScreenSize", &screen_size);
        self.shader_program
            .bind_uniform("uSize", &[text.char_size(), text.char_size()]);
        self.shader_program.bind_uniform("uTexture", &false);
        self.shader_program.bind_uniform("uHasTex", &true);
        self.shader_program.bind_uniform("uColor", text.color());

        let mut offsetx = pos.x;
        let mut offsety = pos.y;

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::Enable(gl::BLEND);
        }
        for ch in text.get().chars() {
            if ch == '\n' {
                offsety += text.char_size() as i32;
                offsetx = pos.x;
                continue;
            }
            self.shader_program
                .bind_uniform("uOffset", &[offsetx, offsety]);
            let charcode = ch as usize;
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, ft.textures[charcode].id);
            }
            self.vbo.draw();
            offsetx += ((ft.textures[charcode].width as u32 + TEXT_MARGIN) as f32*(text.char_size() as f32/CHAR_BIT_WIDTH as f32)) as i32;
        }
    }
}
