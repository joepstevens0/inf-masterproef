
mod vector2;
mod vector3;
mod vector4;
mod mat4;
mod quaternion;
mod test;

pub use vector2::*;
pub use vector3::*;
pub use vector4::*;

pub type Vec2 = vector2::Vector2<i32>;
pub type Vec3 = vector3::Vector3<i32>;
pub type Vecf3 = vector3::Vector3<f32>;
pub type Vecf4 = vector4::Vector4<f32>;
pub type Vecf2 = vector2::Vector2<f32>;
pub type Vecu2 = vector2::Vector2<u32>;
pub type Vecu3 = vector3::Vector3<u32>;
pub type Matf4 = mat4::Mat4<f32>;

pub use quaternion::*;