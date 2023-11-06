use crate::{Size, Pos};

use super::{cursor::CursorState, WindowController, CursorController, content::{ContentState, ContentController}, KeyboardState, KeyboardController};



pub struct WindowState {
    cursor: CursorState,
    keyboard: KeyboardState,
    content: ContentState,
    size: Size,
    pos: Pos
}
impl WindowState {
    pub fn new(size: Size) -> Self {
        Self {
            cursor: CursorState::new(),
            keyboard: KeyboardState::new(),
            content: ContentState::new(),
            pos: Pos::default(),
            size
        }
    }

    pub fn controller(&mut self) -> WindowController{
        WindowController::new(self)
    }

    pub fn controller_cursor(&mut self) -> CursorController{
        self.cursor.controller()
    }
    pub fn controller_keyboard(&mut self) -> KeyboardController{
        self.keyboard.controller()
    }

    pub fn controller_content(&mut self) -> ContentController{
        self.content.controller()
    }

    pub fn cursor(&self) -> &CursorState{
        &self.cursor
    }
    pub fn keyboard(&self) -> &KeyboardState{
        &self.keyboard
    }
    pub fn content(&self) -> &ContentState{
        &self.content
    }

    pub fn pos(&self) -> Pos{
        self.pos
    }
    pub fn size(&self) -> Size{
        self.size
    }

    pub(super) fn set_pos(&mut self, pos: Pos){
        self.pos = pos;
    }
    pub(super) fn set_size(&mut self, size: Size){
        self.size = size;
    }
}
