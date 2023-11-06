use crate::{Vecf4, Matf4};




#[derive(Debug,Clone, Copy, PartialEq)]
pub struct Mat4<T> {
    pub data: [T;16]
}

impl Mat4<f32> {
    pub fn new() -> Self {
        let data = [0.;16];
        let mut this = Self { data};

        // load identity
        this.set(0,0, 1.);
        this.set(1,1, 1.);
        this.set(2,2, 1.);
        this.set(3,3, 1.);
        this
    }

    pub fn set_pos(&mut self, pos: Vecf4){
        self.set(3, 0, pos[0]);
        self.set(3, 1, pos[1]);
        self.set(3, 2, pos[2]);
    }

    pub fn pos(&mut self) -> Vecf4{
        let mut pos = Vecf4::default();
        pos.x = self.get(3, 0);
        pos.y = self.get(3, 1);
        pos.z = self.get(3, 2);
        pos
    }

    pub fn set(&mut self,x: usize, y: usize, value: f32){
        self.data[x*4 + y] = value;
    }
    pub fn get(&self,x: usize, y: usize) -> f32{
        self.data[x*4 + y]
    }
}

impl std::ops::Mul<&Vecf4> for Matf4 {
    type Output = Vecf4;

    fn mul(self, rhs: &Vecf4) -> Self::Output {
        let mut result = Vecf4::default();

        result.x = self.get(0, 0)*rhs.x + self.get(1, 0)*rhs.y + self.get(2, 0)*rhs.z + self.get(3, 0)*rhs.w;
        result.y = self.get(0, 1)*rhs.x + self.get(1, 1)*rhs.y + self.get(2, 1)*rhs.z + self.get(3, 1)*rhs.w;
        result.z = self.get(0, 2)*rhs.x + self.get(1, 2)*rhs.y + self.get(2, 2)*rhs.z + self.get(3, 2)*rhs.w;
        result.w = self.get(0, 3)*rhs.x + self.get(1, 3)*rhs.y + self.get(2, 3)*rhs.z + self.get(3, 3)*rhs.w;
        return result;
    }
}

impl std::ops::Mul<f32> for Matf4 {
    type Output = Matf4;

    fn mul(mut self, rhs: f32) -> Self::Output {
        for v in &mut self.data{
            *v *= rhs;
        }
        return self;
    }
}

impl std::ops::Mul<&Matf4> for Matf4 {
    type Output = Matf4;

    fn mul(self, rhs: &Matf4) -> Self::Output {
        let mut result = Matf4::new();

        for x in 0..4{
            for y in 0..4{
                let mut sum = 0.;
                for k in 0..4{
                    sum += self.get(k, y)*rhs.get(x, k);
                }
                result.set(x, y, sum);
            }
        }
        return result;
    }
}

impl std::ops::Mul<Matf4> for Matf4 {
    type Output = Matf4;

    fn mul(self, rhs: Matf4) -> Self::Output {
        let mut result = Matf4::new();

        for x in 0..4{
            for y in 0..4{
                let mut sum = 0.;
                for k in 0..4{
                    sum += self.get(k, y)*rhs.get(x, k);
                }
                result.set(x, y, sum);
            }
        }
        return result;
    }
}