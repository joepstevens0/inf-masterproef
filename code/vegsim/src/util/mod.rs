
mod boundingvolume;
pub use boundingvolume::*;
mod meshcreate;
pub use meshcreate::*;
pub mod random;

use patutil::{Matf4, Vecf3};

pub fn meter_to_real_length(length_in_meters: f32) -> f32{
    length_in_meters*5.
}


pub fn scale_from_size(size: &Vecf3) -> Matf4{
    let mut scale = Matf4::new();
    scale.set(0, 0, size.x);
    scale.set(1, 1, size.y);
    scale.set(2, 2, size.z);
    scale
}
pub fn translation_from_pos(pos: &Vecf3) -> Matf4{
    let pos = nalgebra_glm::vec3(pos.x, pos.y, pos.z);
    let m = nalgebra_glm::translate(&nalgebra_glm::Mat4::identity(), &pos);
    let mut translate = patutil::Matf4::new();
    translate.data.copy_from_slice(m.data.as_slice());
    translate
}

pub fn rot_vec_around_axis(vec: &Vecf3, axis: &Vecf3, radians: f32) -> Vecf3{
    let axis = nalgebra_glm::vec3(axis.x, axis.y, axis.z);
    let vec = nalgebra_glm::vec3(vec.x, vec.y, vec.z);
    let r = nalgebra_glm::rotate_vec3(&vec, radians, &axis);
    return Vecf3::new(r.x, r.y, r.z);
}

pub fn rot_from_dir(dir: &Vecf3) -> Matf4{
    // apply rotation
    let dir = nalgebra_glm::vec3(dir.x, dir.y, dir.z);
    let mut rot = Matf4::new();
    if dir.x == 0. && dir.z == 0. {
        if dir.z < 0. {
            rot.data.copy_from_slice(nalgebra_glm::rotate(
            &nalgebra_glm::identity(),
            patutil::radians(180.),
            &nalgebra_glm::vec3(1., 0., 0.),
            ).as_slice());
        }
    } else {
        let new_y = nalgebra_glm::normalize(&dir);
        let new_z = nalgebra_glm::normalize(&nalgebra_glm::cross(
            &new_y,
            &nalgebra_glm::vec3(0., 1., 0.),
        ));
        let new_x = nalgebra_glm::cross(&new_y, &new_z);
        let m = nalgebra_glm::Mat3::from_columns(&[new_x, new_y, new_z]);
        rot.data.copy_from_slice(nalgebra_glm::mat3_to_mat4(&m).as_slice());
    }
    rot
}