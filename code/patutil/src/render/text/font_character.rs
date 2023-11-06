pub const CHAR_BYTE_WIDTH: usize = 2;
pub const CHAR_BYTE_HEIGHT: usize = 2*8;
pub const CHAR_BYTE_SIZE: usize = (CHAR_BYTE_WIDTH * CHAR_BYTE_HEIGHT) as usize;
pub const CHAR_BIT_WIDTH: usize = CHAR_BYTE_WIDTH*8;
pub const CHAR_BIT_HEIGHT: usize = CHAR_BYTE_HEIGHT;
pub const CHAR_BIT_SIZE: usize = CHAR_BYTE_SIZE*8;
pub const CHAR_WHITE_SPACE_WIDTH: u32 = 5;

pub struct FontCharacter {
    bytes: [u8; CHAR_BYTE_SIZE],
    width: u32,
}

impl FontCharacter{
    pub fn set_bytes(&mut self, bytes: [u8; CHAR_BYTE_SIZE]){
        self.bytes = bytes;

        // recalculate width
        let right_most = self.get_rightmost_bit();
        let left_most = self.get_leftmost_bit();
        self.width = i32::max(right_most as i32 - left_most as i32 + 1, 0) as u32;
        if self.width == 0{
            self.width = CHAR_WHITE_SPACE_WIDTH;
        }
    }

    pub fn width(&self) -> u32{
        return self.width;
    }

    fn get_rightmost_bit(&self) -> usize{
        for bit_x in (0..CHAR_BIT_WIDTH).rev(){
            for bit_y in 0..CHAR_BIT_HEIGHT{
                if self.get_bit(bit_x, bit_y){
                    return bit_x;
                }
            }
        }
        return 0;
    }
    fn get_leftmost_bit(&self) -> usize{
        for bit_x in 0..CHAR_BIT_WIDTH{
            for bit_y in 0..CHAR_BIT_HEIGHT{
                if self.get_bit(bit_x, bit_y){
                    return bit_x;
                }
            }
        }
        return CHAR_BYTE_WIDTH*8;
    }
    pub fn get_bit(&self, bit_x: usize, bit_y: usize) -> bool{
        let byte_x = bit_x/8;
        let bit_mask =  1 << bit_x%8;

        return (self.bytes[bit_y*CHAR_BYTE_WIDTH + byte_x] & bit_mask) > 0
    }
}

pub const EMPTY_CHARACTER: FontCharacter = FontCharacter {
    bytes: [0; CHAR_BYTE_SIZE],
    width: 0,
};

#[test]
fn font_char_indexing(){
    let ch = EMPTY_CHARACTER;
    
    // try indexing all corners
    assert!(!ch.get_bit(0, 0), "index [{},{}]", 0, 0);
    assert!(!ch.get_bit(CHAR_BIT_HEIGHT - 1, 0), "index [{},{}]", CHAR_BIT_HEIGHT - 1, 0);
    assert!(!ch.get_bit(0, CHAR_BIT_WIDTH - 1), "index [{},{}]", 0, CHAR_BIT_WIDTH - 1);
    assert!(!ch.get_bit(CHAR_BIT_HEIGHT - 1, CHAR_BIT_WIDTH - 1), "index [{},{}]", CHAR_BIT_HEIGHT - 1, CHAR_BIT_WIDTH - 1);
}

#[test]
fn empty_font_char(){
    let bytes = [0; CHAR_BYTE_SIZE];

    // init character
    let mut ch = EMPTY_CHARACTER;
    ch.set_bytes(bytes);

    // width is white space width
    assert_eq!(ch.width, CHAR_WHITE_SPACE_WIDTH, "width of empty character test");

    // test if all spaces 0
    for bit_x in 0..CHAR_BIT_WIDTH{
        for bit_y in 0..CHAR_BIT_HEIGHT{
            assert_eq!(ch.get_bit(bit_y, bit_x), false, "empty test row: {}, column: {}", bit_y, bit_x);
        }
    }
}

#[test]
fn full_font_char(){
    let bytes = [255u8; CHAR_BYTE_SIZE];

    let mut ch = EMPTY_CHARACTER;
    ch.set_bytes(bytes);

    assert_eq!(ch.width, CHAR_BIT_WIDTH as u32);

    // test if all spaces 1
    for bit_x in 0..CHAR_BIT_WIDTH{
        for bit_y in 0..CHAR_BIT_HEIGHT{
            assert_eq!(ch.get_bit(bit_y, bit_x), true, "full test row: {}, column: {}", bit_y, bit_x);
        }
    }
}