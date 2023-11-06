use std::{ops::{Index, IndexMut}, collections::HashMap};

use iced_glutin::glutin;

mod controller;
pub use controller::*;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyState {
    Released,
    StartPress,
    Pressing,
    EndPress,
}

pub type KeyCode = glutin::event::VirtualKeyCode;

pub struct KeyboardState {
    keys: HashMap<KeyCode,KeyState>,
    key_results: Vec<KeyCode>
}

impl KeyboardState {
    pub fn new() -> Self {
        let keys = HashMap::new();
        Self {
            keys,
            key_results: vec![]
        }
    }

    pub fn controller(&mut self) -> KeyboardController{
        KeyboardController::new(self)
    }

    pub fn key_results(&self) -> &Vec<KeyCode>{
        return &self.key_results;
    }

    pub fn pressed_keys(&self) -> Vec<KeyCode>{
        let mut result = vec![];
        for (key, state) in &self.keys{
            if state == &KeyState::StartPress || state == &KeyState::Pressing{
                result.push(*key);
            }
        }
        result
    }

    pub fn is_pressed(&self,key: KeyCode) -> bool{
        return self[key] == KeyState::StartPress || self[key] == KeyState::Pressing;
    }

    pub fn caps_on(&self) -> bool{
        self.is_pressed(KeyCode::LShift) || self.is_pressed(KeyCode::RShift)
    }
}

impl Index<KeyCode> for KeyboardState{
    type Output = KeyState;

    fn index(&self, index: KeyCode) -> &Self::Output {
        let r = self.keys.get(&index);
        if let Some(state) = r{
            return state;
        }
        return &KeyState::Released;
    }
}

impl IndexMut<KeyCode> for KeyboardState{
    fn index_mut(&mut self, index: KeyCode) -> &mut Self::Output {
        // add key if no state yet
        if !self.keys.contains_key(&index){
            self.keys.insert(index, KeyState::Released);
        }
        self.keys.get_mut(&index).unwrap()
    }
}