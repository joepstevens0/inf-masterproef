use crate::Pos;

use super::{CursorState, CursorIcon, MouseButton, MouseButtonState};



pub struct CursorController<'a> {
    state: &'a mut CursorState,
}

impl<'a> CursorController<'a> {
    pub fn new(state: &'a mut CursorState) -> Self {
        Self { state }
    }

    // change cursor icon
    pub fn set_cursor_icon(&mut self, icon: CursorIcon){
        self.state.cursor_icon = icon;
    }

    // mousebutton press event
    pub fn press(&mut self, button: MouseButton){
        #[cfg(debug_assertions)]
        if self.state[button] == MouseButtonState::Released{
            println!("Error windowstate: key <{:?}> pressed, but was not released", button);
        }

        self.state[button] = MouseButtonState::StartPress;
    }

    // mousebutton release event
    pub fn release(&mut self, button: MouseButton){
        #[cfg(debug_assertions)]
        if self.state[button] == MouseButtonState::StartPress || self.state[button] == MouseButtonState::Pressing{
            println!("Error windowstate: key <{:?}> released, but was not pressed", button);
        }
        
        self.state[button] = MouseButtonState::EndPress;
    }

    // release button is pressed
    fn release_if_press(&mut self, button: MouseButton){
        if self.state[button] == MouseButtonState::StartPress || self.state[button] == MouseButtonState::Pressing{
            self.release(button);
        }
    }

    // release all pressed buttons
    pub fn release_all_pressed(&mut self){
        self.release_if_press(MouseButton::Left);
        self.release_if_press(MouseButton::Middle);
        self.release_if_press(MouseButton::Right);
    }

    // change cursor position
    pub fn pos_update(&mut self, position: Pos){
        self.state.position = position;
    }

    // advance state a frame
    pub fn advance(&mut self) {
        // advance buttons
        self.advance_button(MouseButton::Left);
        self.advance_button(MouseButton::Middle);
        self.advance_button(MouseButton::Right);

        // advance cursor icon
        self.state.cursor_icon = CursorIcon::default();
    }

    // advance button state to the next logical state
    fn advance_button(&mut self, button: MouseButton){
        let button_state = &mut self.state[button];
        match button_state {
            MouseButtonState::Released => {}
            MouseButtonState::StartPress => {
                *button_state = MouseButtonState::Pressing;
            }
            MouseButtonState::Pressing => {}
            MouseButtonState::EndPress => {
                *button_state = MouseButtonState::Released;
            }
        }
    }
}
