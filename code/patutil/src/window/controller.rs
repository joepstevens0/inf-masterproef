use crate::{Pos, Size};

use super::{state::WindowState, CursorController, KeyboardController};

pub struct WindowController<'a> {
    state: &'a mut WindowState,
}

impl<'a> WindowController<'a> {
    pub fn new(state: &'a mut WindowState) -> Self {
        Self { state }
    }

    pub fn cursor(&mut self) -> CursorController{
        return self.state.controller_cursor()
    }

    pub fn keyboard(&mut self) -> KeyboardController{
        return self.state.controller_keyboard();
    }

    pub fn advance(&mut self) {
        self.cursor().advance();
        self.keyboard().advance();
    }

    pub fn set_window_pos(&mut self, pos: Pos){
        self.state.set_pos(pos);
    }
    pub fn set_window_size(&mut self, size: Size){
        self.state.set_size(size);
    }
}
