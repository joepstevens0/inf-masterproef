use patutil::{Color, Vec3, Vecf3, Vecu3};

use crate::{util::BoundingVolume, parameters};

type ShadowVoxel = f32;

#[derive(Debug, Clone)]
pub struct ShadowVoxelSet {
    boudingbox: BoundingVolume,
    resolution: Vecu3,
    voxels: Vec<f32>,
}

impl ShadowVoxelSet {
    pub fn new(boudingbox: BoundingVolume, resolution: Vecu3) -> Self {
        let mut voxels = vec![];
        voxels.resize((resolution.x * resolution.y * resolution.z) as usize, 0.);
        Self {
            boudingbox,
            resolution,
            voxels,
        }
    }

    pub fn clear(&mut self) {
        for voxel in &mut self.voxels {
            *voxel = 0.;
        }
    }

    pub fn add_shadow(&mut self, pos: Vecf3) {
        let voxel_p = self
            .boudingbox
            .reverse_interpolate(pos, self.resolution, false);
        let layers = parameters::SHADOW_VOXEL_PIRAMID_LAYERS.min(voxel_p.y as i32 + 1);
        for layer in 0..layers {
            let min_p = voxel_p.to_i32()
                - Vec3::new(layer, 0, layer);
            let max_p = voxel_p.to_i32()
                + Vec3::new(layer + 1, 0, layer + 1);

            let y = voxel_p.y - layer as u32;
            for x in min_p.x..max_p.x {
                for z in min_p.z..max_p.z {
                    let pos = Vecu3::new(x as u32, y, z as u32);
                    let voxel = self.get_voxel_mut(pos);
                    if let Some(voxel) = voxel{
                        *voxel += parameters::SHADOW_VOXEL_A * parameters::SHADOW_VOXEL_B.powf(-layer as f32);
                    }
                }
            }
        }
    }

    pub fn optimal_growth_direction(&self, bud_pos: Vecf3, dir: Vecf3, _theta: f32, r: f32) -> Option<Vecf3>{
        let mut optimal_dir = Vecf3::new(0.,0.,0.);
        for (pos, shadow) in self.get_voxels_in_sphere(bud_pos, r){
            let voxel_dir = (pos - bud_pos).norm();

            optimal_dir -= voxel_dir*(*shadow);
        }

        if optimal_dir == Vecf3::new(0., 0., 0.){
            optimal_dir = dir;
        }
        return Some(optimal_dir.norm());
    }

    pub fn get_light_exposure(&self, pos: Vecf3) -> f32 {
        let voxel = self
            .get_voxel(
                self.boudingbox
                    .reverse_interpolate(pos, self.resolution, false),
            );

        let shadow = *voxel.unwrap_or(&parameters::SHADOW_VOXEL_MAX_SHADOW);

        return f32::max(parameters::SHADOW_VOXEL_C - shadow + parameters::SHADOW_VOXEL_A, 0.);
    }


    fn get_voxel_mut(&mut self, pos: Vecu3) -> Option<&mut ShadowVoxel> {
        if pos.x >= self.resolution.x || pos.z >= self.resolution.z || pos.y >= self.resolution.y{
            return None;
        }
        let index = (pos.y * self.resolution.x* self.resolution.z
            + pos.z * self.resolution.x
            + pos.x) as usize;
        if index >= self.voxels.len() {
            return None;
        }

        return Some(&mut self.voxels[index]);
    }
    fn get_voxel(&self, pos: Vecu3) -> Option<&ShadowVoxel> {
        if pos.x >= self.resolution.x || pos.z >= self.resolution.z || pos.y >= self.resolution.y{
            return None;
        }
        let index = (pos.y * self.resolution.x* self.resolution.z
            + pos.z * self.resolution.x
            + pos.x) as usize;
        return Some(&self.voxels[index]);
    }

    fn get_voxels_in_sphere(&self, point: Vecf3, r: f32) -> Vec<(Vecf3, &ShadowVoxel)>{
        let mut voxels = vec![];

        let min_p = Vecf3::new(point.x - r, point.y - r, point.z - r);
        let max_p = Vecf3::new(point.x + r, point.y + r, point.z + r);

        let step = self.boudingbox.interpolate(Vecu3::new(1,1,1), self.resolution)
        - self.boudingbox.interpolate(Vecu3::new(0,0,0), self.resolution);
        let mut p = min_p;
        while p.x < max_p.x {
            p.y = min_p.y;
            while p.y < max_p.y{
                p.z = min_p.z;
                while p.z < max_p.z {
                    let voxel_dir = p - point;
                    if voxel_dir.length() > r{
                        p.z += step.z;
                        continue;   // point not in range
                    }

                    if let Some(voxel) = self.get_voxel(self.boudingbox.reverse_interpolate(p, self.resolution, false)){
                        voxels.push((p, voxel));
                    } else {
                        voxels.push((p, &parameters::SHADOW_VOXEL_MAX_SHADOW));
                    }
                    p.z += step.z;
                }
                p.y += step.y;
            }
            p.x += step.x;
        }

        return voxels;
    }

    pub fn debug_texture(&self, layer: u32) -> Vec<Color> {
        let mut data: Vec<Color> = vec![];
        data.resize(
            (self.resolution.x * self.resolution.z) as usize,
            Color::new(0, 0, 0, 255),
        );
        if layer >= self.resolution.y{
            return data;
        }
        for x in 0..self.resolution.x {
            for z in 0..self.resolution.z {
                let voxel = self.get_voxel(Vecu3::new(x, layer, z));
                if let Some(voxel) = voxel{
                    let c = 255 - (*voxel * 128.) as u8;
                    data[(z * self.resolution.x + x) as usize] = Color::new(c, c, c, 255);
                }
            }
        }
        return data;
    }
}
