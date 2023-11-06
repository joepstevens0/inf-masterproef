use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use patfile::{pscan, pwrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    it: usize,
}

impl<T> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w, it: 0 }
    }

    pub fn to_vec(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Vector4<f32> {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn norm(&mut self) {
        let len = self.len();
        if len == 0. {
            return;
        }
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
        self.w = self.w / len;
    }

    pub fn abs(mut self) -> Vector4<f32>{
        self.x = self.x.abs();
        self.y = self.y.abs();
        self.z = self.z.abs();
        self.w = self.w.abs();
        self
    }

    pub fn cross(&self, other: Vector4<f32>)-> Vector4<f32>{
        return Vector4::<f32>::new(self.y*other.z - self.z*other.y, self.z*other.x - self.x*other.z, self.x*other.y - self.y*other.x, 1.);
    }
}

impl<
        T: std::ops::Mul<f32, Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + Copy,
    > Vector4<T>
{
    pub fn rotate_x(&self, angle: f32) -> Self {
        Self::new(
            self.x,
            self.y * angle.cos() - self.z * angle.sin(),
            self.y * angle.sin() + self.z * angle.cos(),
            self.w,
        )
    }
    pub fn rotate_y(&self, angle: f32) -> Self {
        Self::new(
            self.x * angle.cos() + self.z * angle.sin(),
            self.y,
            self.z * angle.cos() - self.x * angle.sin(),
            self.w,
        )
    }
    pub fn rotate_z(&self, angle: f32) -> Self {
        Self::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
            self.z,
            self.w,
        )
    }
}
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for &Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4::<T>::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4::<T>::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for Vector4<T> {
    type Output = Vector4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4::<T>::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}


impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector4::<T>::new(
            self.x/ rhs,
            self.y/ rhs,
            self.z/ rhs,
            self.w/ rhs,
        )
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4::<T>::new(
            self.x* rhs,
            self.y* rhs,
            self.z* rhs,
            self.w* rhs,
        )
    }
}

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> std::ops::Mul<Vector4<T>> for Vector4<T> {
    type Output = T;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z + self.w*rhs.w
    }
}

impl<T> std::ops::Index<usize> for Vector4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 Index out of bounds"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 Index out of bounds"),
        }
    }
}

impl<T: Copy> Iterator for Vector4<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = match self.it {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => return None,
        };
        self.it += 1;
        Some(r)
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for &Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4::<T>::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign<Vector4<T>> for Vector4<T> {
    fn add_assign(&mut self, rhs: Vector4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T> From<[T; 4]> for Vector4<T> {
    fn from(data: [T; 4]) -> Self {
        let mut v = data.into_iter();
        unsafe {
            let x = v.next().unwrap_unchecked();
            let y = v.next().unwrap_unchecked();
            let z = v.next().unwrap_unchecked();
            let w = v.next().unwrap_unchecked();
            Vector4::<T>::new(x, y, z, w)
        }
    }
}

impl<T> FromStr for Vector4<T>
where
    T: FromStr + Display + Default,
    <T as FromStr>::Err: ::std::fmt::Debug,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = T::default();
        let mut y= T::default();
        let mut z= T::default();
        let mut w= T::default();

        let mut it = s.bytes().into_iter();
        pscan!(&mut it => "[{},{},{},{}]", x,y,z,w).unwrap();

        Ok(Vector4::new(x,y,z,w))
    }
}

impl<T: Display> Display for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer: &mut dyn std::fmt::Write = f;
        pwrite!("[{},{},{},{}]" => writer, &self.x, &self.y, &self.z, &self.w).unwrap();
        Ok(())
    }
}


impl<T: Default> Default for Vector4<T>{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default(), w: Default::default(), it: Default::default() }
    }
}