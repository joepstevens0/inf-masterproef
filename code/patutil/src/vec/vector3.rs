

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Vector3<T>{
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vector3<T>{
    pub const fn new(x: T, y: T, z: T) -> Self{
        Self{x, y, z}
    }
}

impl Vector3<f32>{
    pub fn ceil(self) -> Vector3<i32>{
        Vector3 { x: self.x.ceil() as i32, y: self.y.ceil() as i32, z: self.z.ceil() as i32 }
    }

    pub fn floor(self) -> Vector3<i32>{
        Vector3 { x: self.x.floor() as i32, y: self.y.floor() as i32, z: self.z.floor() as i32 }
    }

    pub fn norm(mut self) -> Self{
        let len = self.length();
        if len == 0. {
            return self;
        }
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
        self
    }
    pub fn cross(&self, other: Vector3<f32>)-> Vector3<f32>{
        return Vector3::<f32>::new(self.y*other.z - self.z*other.y, self.z*other.x - self.x*other.z, self.x*other.y - self.y*other.x);
    }

    // return angle in radians
    pub fn angle_between(&self, other: &Vector3<f32>) -> f32{
        ((self*other)/(self.length()*other.length())).acos()
    }
}

impl Vector3<i32>{
    pub fn to_float(self) -> Vector3<f32>{
        Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
    pub fn to_u32(self)-> Vector3<u32>{
        Vector3 { x: self.x.abs() as u32, y: self.y.abs() as u32, z: self.z.abs() as u32 }
    }
}
impl Vector3<u32>{
    pub fn to_float(self) -> Vector3<f32>{
        Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
    pub fn to_i32(self)-> Vector3<i32>{
        Vector3 { x: self.x as i32, y: self.y as i32, z: self.z as i32 }
    }
}

impl<T: Into<f32>> Vector3<T>{
    pub fn length(self) -> f32{
        return (self.x.into().powf(2.) + self.y.into().powf(2.) + self.z.into().powf(2.)).sqrt();
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.x)
         .field(&self.y)
         .field(&self.z)
         .finish()
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for &Vector3<T>{
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x + rhs.x,self.y + rhs.y, self.z + rhs.z)
    }
}
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Vector3<T>{
    type Output = Vector3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x + rhs.x,self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign for Vector3<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for &Vector3<T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x - rhs.x,self.y - rhs.y, self.z - rhs.z)
    }
}
impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for Vector3<T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::<T>::new(self.x - rhs.x,self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: std::ops::SubAssign> std::ops::SubAssign for Vector3<T>{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vector3<T>{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::<T>::new(self.x*rhs,self.y*rhs, self.z*rhs)
    }
}
impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Vector3<T>{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3::<T>::new(self.x/rhs,self.y/rhs, self.z/rhs)
    }
}

impl<T: std::ops::Neg<Output = T> + Copy> std::ops::Neg for &Vector3<T>{
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        Vector3::<T>::new(-self.x,-self.y,-self.z)
    }
}

impl<T> std::ops::Index<i32> for Vector3<T>{
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec2 Index out of bounds")
        }
    }
}

impl<T> std::ops::IndexMut<i32> for Vector3<T>{

    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec2 Index out of bounds")
        }
    }
}

impl<T: std::ops::Rem<Output=T> + Clone> std::ops::Rem<T> for Vector3<T>{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Vector3{x: self.x % rhs.clone(), y: self.y % rhs.clone(), z: self.z % rhs}
    }
}
impl<T: std::ops::RemAssign + Clone> std::ops::RemAssign<T> for Vector3<T>{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs.clone();
        self.z %= rhs;
    }
}

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> std::ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
}

impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> std::ops::Mul<&Vector3<T>> for &Vector3<T> {
    type Output = T;

    fn mul(self, rhs: &Vector3<T>) -> Self::Output {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
}


impl<T: Clone> From<[T;3]> for Vector3<T>{
    fn from(data: [T;3]) -> Self {
        Vector3 { x: data[0].clone(), y: data[1].clone(), z: data[2].clone() }
    }
}

impl From<Vector3<i32>> for Vector3<f32>{
    fn from(data: Vector3<i32>) -> Self {
        Vector3 { x: data.x as f32, y: data.y as f32, z: data.z as f32 }
    }
}
impl From<Vector3<f32>> for Vector3<i32>{
    fn from(data: Vector3<f32>) -> Self {
        Vector3 { x: data.x as i32, y: data.y as i32, z: data.z as i32 }
    }
}


impl<T: Default> Default for Vector3<T>{
    fn default() -> Self {
        Self { x: T::default(), y: T::default(), z: T::default() }
    }
}

impl<
        T: std::ops::Mul<f32, Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Add<Output = T>
            + Copy,
    > Vector3<T>
{
    pub fn rotate_x(&self, angle: f32) -> Self {
        Self::new(
            self.x,
            self.y * angle.cos() - self.z * angle.sin(),
            self.y * angle.sin() + self.z * angle.cos(),
        )
    }
    pub fn rotate_y(&self, angle: f32) -> Self {
        Self::new(
            self.x * angle.cos() + self.z * angle.sin(),
            self.y,
            self.z * angle.cos() - self.x * angle.sin(),
        )
    }
    pub fn rotate_z(&self, angle: f32) -> Self {
        Self::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
            self.z,
        )
    }
}