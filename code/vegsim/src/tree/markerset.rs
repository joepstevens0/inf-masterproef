
use patutil::{Vecf3, Vecu3};

use crate::util::{BoundingVolume, random::Random};

#[derive(Debug, Clone, Copy)]
pub struct Marker{
    pub claimed_bud: Option<u32>,
    pub distance_to_claimed: f32,
    pub position: Vecf3
}

impl Marker {
    pub fn new(position: Vecf3) -> Self { Self { claimed_bud: None, distance_to_claimed: f32::MAX, position } }
    pub fn reset(&mut self){
        self.claimed_bud = None;
        self.distance_to_claimed = f32::MAX;
    }
    pub fn claim(&mut self, bud_id: u32, dist: f32){
        self.claimed_bud = Some(bud_id);
        self.distance_to_claimed = dist;
    }

}

#[derive(Debug, Clone)]
pub struct MarkerSet {
    markers: Vec<Marker>,
    resolution: Vecu3,
    bounding_volume: BoundingVolume
}

impl MarkerSet {
    pub fn new(bounding_volume: BoundingVolume, resolution: Vecu3) -> Self {
        let mut markers = Vec::new();

        let step = bounding_volume.interpolate(Vecu3::new(1,1,1), resolution)
        - bounding_volume.interpolate(Vecu3::new(0,0,0), resolution);

        for y in 0..resolution.y{
            for z in 0..resolution.z{
                for x in 0..resolution.x{
                    let mut pos = bounding_volume.interpolate(Vecu3::new(x, y, z), resolution);

                    // add random offset on positons
                    pos.x += step.x*Random::rand();
                    pos.y += step.y*Random::rand();
                    pos.z += step.z*Random::rand();

                    markers.push(Marker::new(pos));
                }
            }
        }

        Self {markers, resolution, bounding_volume}
    }

    pub fn get_all_marked_points(&self)-> Vec<&Marker>{
        let mut markers = vec![];
        for marker in self.markers.iter(){
            if marker.claimed_bud.is_some() && marker.claimed_bud != Some(0) {
                markers.push(marker);
            }
        }
        markers
    }

    pub fn reset(&mut self){
        for marker in &mut self.markers{
            marker.reset();
        }
    }

    pub fn set_markers_in_cone(&mut self, id: u32, point: Vecf3, dir: Vecf3, theta: f32, r: f32) -> u32{
        // println!("set markers: pos({:?}) dir({:?}) {},{}", point, dir, theta, r);
        let mut total_marked = 0;
        let cone_markers_positions = self.get_markers_in_cone_positions(point, dir, theta, r);
        // println!("cone markers: {}", cone_markers_positions.len());
        for marker_p in cone_markers_positions{
            if let Some(marker) = self.get_marker_mut(marker_p){
                let dist = (marker.position - point).length();
                if marker.distance_to_claimed > dist{
                    marker.claim(id, dist);
                    total_marked += 1;
                }
            }
        }
        return total_marked;
    }

    pub fn remove_markers_in_sphere(&mut self, point: Vecf3, r: f32){
        let sphere_markers_positions = self.get_markers_in_sphere_positions(point,  r);
        for marker_p in sphere_markers_positions{
            if let Some(marker) = self.get_marker_mut(marker_p){
                marker.claim(0, 0.);
            }
        }
    }

    pub fn total_markers_for_id_in_cone(&self, id: u32, point: Vecf3, dir: Vecf3, theta: f32, r: f32) -> u32{
        let marker_positions = self.get_markers_in_cone(point, dir, theta, r);
        let mut total_markers = 0;

        for marker in marker_positions{
            if marker.claimed_bud == Some(id){
                total_markers += 1;
            }
        }
        total_markers
    }

    pub fn markers_dir_for_id_in_cone(&self, id: u32, point: Vecf3, dir: Vecf3, theta: f32, r: f32) -> Option<Vecf3>{
        let markers = self.get_markers_in_cone(point, dir, theta, r);

        if markers.len() <= 0{
            return None;    // no markers, no direction
        }

        let mut marker_dir = Vecf3::new(0.,0.,0.);
        for marker in markers{
            if marker.claimed_bud == Some(id){
                let dir = (marker.position - point).norm();
                marker_dir += dir;
            }
        }
        return Some(marker_dir.norm());
    }

    fn get_markers_in_cone(&self, point: Vecf3, dir: Vecf3, theta: f32, r: f32) -> Vec<&Marker>{
        let sphere_markers = self.get_markers_in_sphere(point, r);

        let mut cone_markers = vec![];
        for marker in sphere_markers{
            let marker_dir = marker.position - point;
            if marker_dir.angle_between(&dir) > theta{
                continue;   // point outside cone
            }
            cone_markers.push(marker);
        }

        return cone_markers;
    }
    fn get_markers_in_cone_positions(&self, point: Vecf3, dir: Vecf3, theta: f32, r: f32) -> Vec<Vecu3>{
        let sphere_marker_positions = self.get_markers_in_sphere_positions(point, r);

        let mut cone_markers_positions = vec![];
        for marker_p in sphere_marker_positions{
            let marker = self.get_marker(marker_p);

            if let Some(marker) = marker{
                let marker_dir = marker.position - point;
                if marker_dir.angle_between(&dir) > theta{
                    continue;   // point outside cone
                }
                cone_markers_positions.push(marker_p);
            }
        }

        return cone_markers_positions;
    }

    fn get_marker(&self, pos: Vecu3) -> Option<&Marker> {
        if pos.x >= self.resolution.x || pos.z >= self.resolution.z || pos.y >= self.resolution.y{
            return None;
        }
        let index = (pos.y * self.resolution.x* self.resolution.z
            + pos.z * self.resolution.x
            + pos.x) as usize;
        return Some(&self.markers[index]);
    }
    fn get_marker_mut(&mut self, pos: Vecu3) -> Option<&mut Marker> {
        if pos.x >= self.resolution.x || pos.z >= self.resolution.z || pos.y >= self.resolution.y{
            return None;
        }
        let index = (pos.y * self.resolution.x* self.resolution.z
            + pos.z * self.resolution.x
            + pos.x) as usize;
        return Some(&mut self.markers[index]);
    }


    fn get_markers_in_sphere(&self, point: Vecf3, r: f32) -> Vec<&Marker>{
        let mut markers = vec![];

        let min_p = Vecf3::new(point.x - r, point.y - r, point.z - r);
        let max_p = Vecf3::new(point.x + r, point.y + r, point.z + r);

        let step = self.bounding_volume.interpolate(Vecu3::new(1,1,1), self.resolution)
        - self.bounding_volume.interpolate(Vecu3::new(0,0,0), self.resolution);
        let mut p = min_p;
        // println!("step{:?} min{:?} max{:?}", step, min_p, max_p);
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

                    if let Some(marker) = self.get_marker(self.bounding_volume.reverse_interpolate(p, self.resolution, false)){
                        markers.push(marker);
                    }
                    p.z += step.z;
                }
                p.y += step.y;
            }
            p.x += step.x;
        }

        return markers;
    }

    fn get_markers_in_sphere_positions(&self, point: Vecf3, r: f32) -> Vec<Vecu3>{
        let mut marker_positions = vec![];

        let min_p = Vecf3::new(point.x - r, point.y - r, point.z - r);
        let max_p = Vecf3::new(point.x + r, point.y + r, point.z + r);

        let step = self.bounding_volume.interpolate(Vecu3::new(1,1,1), self.resolution)
        - self.bounding_volume.interpolate(Vecu3::new(0,0,0), self.resolution);
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

                    let pos = self.bounding_volume.reverse_interpolate(p, self.resolution, false);
                    marker_positions.push(pos);
                    p.z += step.z;
                }
                p.y += step.y;
            }
            p.x += step.x;
        }

        return marker_positions;
    }
}
