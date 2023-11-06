use std::{sync::{Mutex, Arc, MutexGuard}, fmt::Debug};

use crate::{
    {Vecf2, Vecf4},
    window::WindowState,
    Rect,
};

const DEFAULT_RIGHT: Vecf4 = Vecf4::new(1., 0., 0., 0.);
const DEFAULT_UP: Vecf4 = Vecf4::new(0., 1., 0., 0.);
const DEFAULT_FRONT: Vecf4 = Vecf4::new(0., 0., 1., 0.);

mod freecam;
pub use freecam::*;
mod followcam;
pub use followcam::*;

#[derive(Clone,Debug)]
pub struct CamRef{
    cam: Arc<Mutex<dyn Camera>>
}

impl CamRef {
    pub fn lock<'a>(&'a mut self)-> MutexGuard<'a, dyn Camera + 'static>{
        self.cam.lock().unwrap()
    }
}
impl<T: Camera + 'static> From<T> for CamRef{
    fn from(cam: T) -> Self {
        CamRef { cam: Arc::new(Mutex::new(cam)) }
    }
}

pub trait Camera: Send + Sync + Debug {
    fn base(&self) -> &CameraBase;
    fn base_mut(&mut self) -> &mut CameraBase;
    fn animate(&mut self) -> bool {
        false
    }
    fn pos(&self) -> Vecf4 {
        self.base().pos()
    }
    fn front(&self) -> Vecf4 {
        self.base().front()
    }
    fn right(&self) -> Vecf4 {
        self.base().right()
    }
    fn up(&self) -> Vecf4 {
        self.base().up()
    }
    fn move_cam(&mut self, v: &Vecf4) {
        self.base_mut().move_cam(v)
    }
    fn set_pos(&mut self, pos: Vecf4) {
        self.base_mut().set_pos(pos)
    }

    fn rotate_cam(&mut self, v: &Vecf2) {
        self.base_mut().rotate_cam(v);
    }

    fn update(&mut self, _window_state: &WindowState, _rect: Rect) {}
}
#[derive(Debug, Clone)]
pub struct CameraBase {
    pos: Vecf4,
    front: Vecf4,
    right: Vecf4,
    up: Vecf4,

    x_rot: f32,
    y_rot: f32,
}

impl CameraBase {
    pub fn new() -> Self {
        Self {
            pos: Vecf4::new(0., 0., 0., 0.),
            front: DEFAULT_FRONT,
            right: DEFAULT_RIGHT,
            up: DEFAULT_UP,
            x_rot: 0.,
            y_rot: 0.,
        }
    }
    pub fn pos(&self) -> Vecf4 {
        self.pos
    }
    pub fn front(&self) -> Vecf4 {
        self.front
    }
    pub fn right(&self) -> Vecf4 {
        self.right
    }
    pub fn up(&self) -> Vecf4 {
        self.up
    }

    pub fn move_cam(&mut self, v: &Vecf4) {
        self.pos = &self.pos + v;
    }

    pub fn set_pos(&mut self, pos: Vecf4) {
        self.pos = pos;
    }

    pub fn rotate_cam(&mut self, v: &Vecf2) {
        self.rotate_y(v[0] as f32);
        self.rotate_x(v[1] as f32);
    }

    fn rotate_y(&mut self, angle: f32) {
        self.y_rot += angle;
        self.update_dirs();
    }

    fn rotate_x(&mut self, angle: f32) {
        self.x_rot = (self.x_rot + angle).min(90.).max(-90.);
        self.update_dirs();
    }

    fn update_dirs(&mut self) {
        self.right = DEFAULT_RIGHT.rotate_x(self.x_rot.to_radians());
        self.front = DEFAULT_FRONT.rotate_x(self.x_rot.to_radians());
        self.up = DEFAULT_UP.rotate_x(self.x_rot.to_radians());

        self.right = self.right.rotate_y(self.y_rot.to_radians());
        self.front = self.front.rotate_y(self.y_rot.to_radians());
        self.up = self.up.rotate_y(self.y_rot.to_radians());
    }
}
