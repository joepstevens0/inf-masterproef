use std::{fmt::Display, str::FromStr};

use patfile::{pwrite, pscan};

use crate::{Vecf4, Vecf3};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion{
    quat: glam::Quat
}

impl Quaternion {
    pub fn to_vec(self) -> Vecf4{
        self.quat.to_array().into()
    }

    pub fn to_axis_angle(self) -> (Vecf3, f32){
        let (axis, angle) = self.quat.to_axis_angle();
        (axis.to_array().into(), angle)
    }

    pub fn rotate_x(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_x(radians);
        self.quat = self.quat.mul_quat(rot);
        self
    }
    pub fn rotate_y(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_y(radians);
        self.quat = self.quat.mul_quat(rot);
        self
    }
    pub fn rotate_z(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_z(radians);
        self.quat = self.quat.mul_quat(rot);
        self
    }

    pub fn rotate_local_x(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_x(radians);
        self.quat = rot.mul_quat(self.quat);
        self
    }
    pub fn rotate_local_y(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_y(radians);
        self.quat = rot.mul_quat(self.quat);
        self
    }
    pub fn rotate_local_z(mut self, radians: f32) -> Self{
        let rot = glam::Quat::from_rotation_z(radians);
        self.quat = rot.mul_quat(self.quat);
        self
    }
}

impl From<(Vecf3, f32)> for Quaternion{
    fn from(axis_angle: (Vecf3, f32)) -> Self {
        Quaternion { quat: glam::Quat::from_axis_angle(glam::Vec3::from_array([axis_angle.0.x,axis_angle.0.y,axis_angle.0.z]), axis_angle.1) }
    }
}
impl Default for Quaternion{
    fn default() -> Self {
        Self { quat: glam::Quat::default()}
    }
}

impl Display for Quaternion{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer: &mut dyn std::fmt::Write = f;
        pwrite!("[{},{},{},{}]" => writer, &self.quat.x, &self.quat.y, &self.quat.z, &self.quat.w).unwrap();
        Ok(())
    }
}

impl FromStr for Quaternion{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = f32::default();
        let mut y = f32::default();
        let mut z = f32::default();
        let mut w = f32::default();

        let mut it = s.bytes().into_iter();
        pscan!(&mut it => "[{},{},{},{}]", x,y,z,w).unwrap();

        Ok(Quaternion{quat: glam::Quat::from_array([x,y,z,w])})
    }
}