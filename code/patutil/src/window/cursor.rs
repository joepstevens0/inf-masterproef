use std::ops::{Index, IndexMut};

pub type MouseButton = glutin::event::MouseButton;

use crate::Pos;

mod controller;
pub use controller::*;
use iced_glutin::glutin;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseButtonState {
    Released,
    StartPress,
    Pressing,
    EndPress,
}

pub type CursorIcon = glutin::window::CursorIcon;


pub struct CursorState {
    left_button: MouseButtonState,
    right_button: MouseButtonState,
    middle_button: MouseButtonState,
    position: Pos,
    cursor_icon: CursorIcon,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            left_button: MouseButtonState::Released,
            right_button: MouseButtonState::Released,
            middle_button: MouseButtonState::Released,
            position: Pos { x: -1, y: -1 },
            cursor_icon: CursorIcon::Default,
        }
    }

    pub fn icon(&self) -> CursorIcon{
        self.cursor_icon
    }
    pub fn pos(&self) -> Pos{
        self.position
    }

    pub fn controller(&mut self) -> CursorController{
        CursorController::new(self)
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool{
        return self[button] == MouseButtonState::StartPress || self[button] == MouseButtonState::Pressing;
    }

    
}

impl Index<MouseButton> for CursorState{
    type Output = MouseButtonState;

    fn index(&self, index: MouseButton) -> &Self::Output {
        match index {
            MouseButton::Left => {return &self.left_button},
            MouseButton::Right => {return &self.right_button},
            MouseButton::Middle => {return &self.middle_button},
            MouseButton::Other(_) => todo!(),
        }
    }
}

impl IndexMut<MouseButton> for CursorState{
    fn index_mut(&mut self, index: MouseButton) -> &mut Self::Output {
        match index {
            MouseButton::Left => {return &mut self.left_button},
            MouseButton::Right => {return &mut self.right_button},
            MouseButton::Middle => {return &mut self.middle_button},
            MouseButton::Other(_) => todo!(),
        }
    }
}