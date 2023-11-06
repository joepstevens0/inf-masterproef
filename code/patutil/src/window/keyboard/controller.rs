use super::{KeyState, KeyboardState, KeyCode};

pub struct KeyboardController<'a> {
    state: &'a mut KeyboardState
}

impl<'a> KeyboardController<'a> {
    pub fn new(state: &'a mut KeyboardState) -> Self {
        Self { state }
    }

    fn push_key_result(&mut self, key: KeyCode){
        self.state.key_results.push(key);
    }

    /// key press event
    pub fn press(&mut self, key: KeyCode) {
        if self.state[key] == KeyState::Released{
            self.state[key] = KeyState::StartPress;
        }
        self.push_key_result(key);
    }

    /// key release event
    pub fn release(&mut self, key: KeyCode) {

        self.state[key] = KeyState::EndPress;
    }

    /// release key is pressed
    fn release_if_press(&mut self, key: KeyCode) {
        if self.state[key] == KeyState::StartPress || self.state[key] == KeyState::Pressing {
            self.release(key);
        }
    }

    /// release all pressed keys
    pub fn release_all_pressed(&mut self) {
        let mut keys = vec![];
        for key in self.state.keys.keys(){
            keys.push(*key);
        }
        for key in keys{
            self.release_if_press(key)
        }
    }

    // advance state a frame
    pub fn advance(&mut self) {
        // advance keys
        let mut keys = vec![];
        for key in self.state.keys.keys(){
            keys.push(*key);
        }
        for key in keys{
            self.advance_key(key)
        }
        self.state.key_results.clear();
    }

    /// advance button state to the next logical state
    fn advance_key(&mut self, key: KeyCode) {
        let key_state = &mut self.state[key];
        match key_state {
            KeyState::Released => {}
            KeyState::StartPress => {
                *key_state = KeyState::Pressing;
            }
            KeyState::Pressing => {}
            KeyState::EndPress => {
                *key_state = KeyState::Released;
            }
        }
    }
}
