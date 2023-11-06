use patutil::{Vecf3, Vecu3};

#[derive(Debug, Clone, Copy)]
pub struct BoundingVolume {
    min_pos: Vecf3,
    max_pos: Vecf3,
}

impl BoundingVolume {
    pub fn new() -> Self {
        Self {
            min_pos: Vecf3::new(0., 0., 0.),
            max_pos: Vecf3::new(0., 0., 0.),
        }
    }

    pub fn include_point(&mut self, point: Vecf3){
        self.min_pos.x = self.min_pos.x.min(point.x);
        self.min_pos.y = self.min_pos.y.min(point.y);
        self.min_pos.z = self.min_pos.z.min(point.z);

        self.max_pos.x = self.max_pos.x.max(point.x);
        self.max_pos.y = self.max_pos.y.max(point.y);
        self.max_pos.z = self.max_pos.z.max(point.z);
    }

    pub fn merge(&self, other: BoundingVolume) -> BoundingVolume{
        let mut result = Self::new();

        result.min_pos.x = self.min_pos.x.min(other.min_pos.x);
        result.min_pos.y = self.min_pos.y.min(other.min_pos.y);
        result.min_pos.z = self.min_pos.z.min(other.min_pos.z);

        result.max_pos.x = self.max_pos.x.max(other.max_pos.x);
        result.max_pos.y = self.max_pos.y.max(other.max_pos.y);
        result.max_pos.z = self.max_pos.z.max(other.max_pos.z);

        return result;
    }

    pub fn min_pos(&self) -> Vecf3 {
        self.min_pos
    }

    pub fn max_pos(&self) -> Vecf3 {
        self.max_pos
    }

    pub fn includes(&self, point: Vecf3)->bool{
        self.min_pos.x <= point.x && point.x <= self.max_pos.x &&
        self.min_pos.y <= point.y && point.y <= self.max_pos.y &&
        self.min_pos.z <= point.z && point.z <= self.max_pos.z
    }

    pub fn interpolate(&self, value: Vecu3, resolution: Vecu3) -> Vecf3{
        let x = self.interpolate_value(value.x, resolution.x, self.min_pos.x, self.max_pos.x);
        let y = self.interpolate_value(value.y, resolution.y, self.min_pos.y, self.max_pos.y);
        let z = self.interpolate_value(value.z, resolution.z, self.min_pos.z, self.max_pos.z);

        return Vecf3::new(x,y,z);
    }

    fn interpolate_value(&self, value: u32, resolution: u32, min: f32, max: f32) -> f32{
        if value > resolution{
            panic!("Value can be at most resolution.");
        }
        let factor = value as f32/resolution as f32;
        return min + factor*(max-min);
    }

    pub fn reverse_interpolate(&self, value: Vecf3, resolution: Vecu3, ceil:bool) -> Vecu3{
        let x = self.reverse_interpolate_range(value.x, resolution.x, ceil, self.min_pos.x, self.max_pos.x);
        let y = self.reverse_interpolate_range(value.y, resolution.y, ceil, self.min_pos.y, self.max_pos.y);
        let z = self.reverse_interpolate_range(value.z, resolution.z, ceil, self.min_pos.z, self.max_pos.z);

        return Vecu3::new(x,y,z);
    }
    fn reverse_interpolate_range(&self, value: f32, resolution: u32, ceil:bool, min: f32, max: f32) -> u32{
        let step = (max - min) / resolution as f32;
        if ceil{
            return (((value - min) / step).ceil() as u32).clamp(0, resolution - 1);
        }

        return (((value - min) / step).floor() as u32).clamp(0, resolution - 1);
    }
}
