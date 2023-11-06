use std::time::Instant;

use crate::{
    camera::{Camera, CameraBase},
    window::{MouseButton, MouseButtonState, KeyCode, KeyState},
    Pos, Rect, Vecf2, Vecf4,
};

#[derive(Debug)]
pub struct FollowCam {
    base: CameraBase,
    move_vec: Vecf4,
    focus: bool,
    cursor_last_pos: Pos,
    last_render: Instant,
    pressed: bool,
}

impl Camera for FollowCam {
    fn base(&self) -> &CameraBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut CameraBase {
        &mut self.base
    }
    fn animate(&mut self) -> bool {
        self.update()
    }

    fn update(&mut self, window_state: &crate::window::WindowState, rect: Rect) {
        let cursor_pos = window_state.cursor().pos();
        if self.focus {
            let diff = cursor_pos - self.cursor_last_pos;
            let v: Vecf2 = Vecf2::new(diff.x as f32, diff.y as f32);
            self.rotate_cam(&v);
        }
        self.cursor_last_pos = cursor_pos.clone();

        // check button presses
        match window_state.cursor()[MouseButton::Left] {
            MouseButtonState::StartPress => {
                if !self.pressed && rect.is_inside(cursor_pos) {
                    self.pressed = true;
                    self.focus = true;
                    self.cursor_last_pos = cursor_pos;
                }
            }
            MouseButtonState::EndPress => {
                if self.pressed {
                    self.pressed = false;
                    self.focus = false;
                    self.cursor_last_pos = cursor_pos;
                }
            }
            _ => {}
        }

        // check keyboard pressed
        if rect.is_inside(cursor_pos){
            if window_state.keyboard()[KeyCode::A] == KeyState::StartPress{
        
                self.move_vec[0] = -Self::MOVE_VAL;
            }
            if window_state.keyboard()[KeyCode::D] == KeyState::StartPress{
                self.move_vec[0] = Self::MOVE_VAL;
            }
            if window_state.keyboard()[KeyCode::W] == KeyState::StartPress{
                self.move_vec[2] = Self::MOVE_VAL;
            }
            if window_state.keyboard()[KeyCode::S] == KeyState::StartPress{
                self.move_vec[2] = -Self::MOVE_VAL;
            }
            if window_state.keyboard()[KeyCode::Space] == KeyState::StartPress{
                self.move_vec[1] = Self::MOVE_VAL;
            }
            if window_state.keyboard()[KeyCode::LShift] == KeyState::StartPress{
                self.move_vec[1] = -Self::MOVE_VAL;
            }
        }

        // check keyboard released
        if window_state.keyboard()[KeyCode::A] == KeyState::EndPress{
            self.move_vec[0] = self.move_vec[0].max(0.);
        }
        if window_state.keyboard()[KeyCode::D] == KeyState::EndPress{
            self.move_vec[0] = self.move_vec[0].min(0.);
        }
        if window_state.keyboard()[KeyCode::W] == KeyState::EndPress{
            self.move_vec[2] = self.move_vec[2].min(0.);
        }
        if window_state.keyboard()[KeyCode::S] == KeyState::EndPress{
            self.move_vec[2] = self.move_vec[2].max(0.);
        }
        if window_state.keyboard()[KeyCode::Space] == KeyState::EndPress{
            self.move_vec[1] = self.move_vec[1].min(0.);
        }
        if window_state.keyboard()[KeyCode::LShift] == KeyState::EndPress{
            self.move_vec[1] = self.move_vec[1].max(0.);
        }

        self.update();
    }
}

impl FollowCam {
    pub fn new() -> Self {
        Self {
            base: CameraBase::new(),
            move_vec: Vecf4::new(0., 0., 0., 0.),
            focus: false,
            cursor_last_pos: Pos::default(),
            last_render: Instant::now(),
            pressed: false,
        }
    }

    pub fn update(&mut self) -> bool {
        if self.last_render.elapsed() > std::time::Duration::from_millis(20) {
            let y_move = &self.up() * self.move_vec[1];
            let x_move = &self.right() * self.move_vec[0];
            let mut z_move = self.front();
            z_move[1] = 0.;
            z_move.norm();
            z_move = &z_move * self.move_vec[2];
            let cam_move = &x_move + &(&y_move + &z_move);
            self.move_cam(&(&cam_move * self.last_render.elapsed().as_secs_f32()));

            self.last_render = Instant::now();
            return true;
        }
        false
    }

    const MOVE_VAL: f32 = 10.;
}
