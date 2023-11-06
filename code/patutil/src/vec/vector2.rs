

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Vector2<T>{
    pub x: T,
    pub y: T
}

impl<T> Vector2<T>{
    pub const fn new(x: T, y: T) -> Self{
        Self{x, y}
    }
}

impl Vector2<f32>{
    pub fn ceil(self) -> Vector2<i32>{
        Vector2 { x: self.x.ceil() as i32, y: self.y.ceil() as i32 }
    }

    pub fn floor(self) -> Vector2<i32>{
        Vector2 { x: self.x.floor() as i32, y: self.y.floor() as i32 }
    }
}

impl Vector2<i32>{
    pub fn to_float(self) -> Vector2<f32>{
        Vector2 { x: self.x as f32, y: self.y as f32 }
    }
    pub fn to_u32(self)-> Vector2<u32>{
        Vector2 { x: self.x.abs() as u32, y: self.y.abs() as u32 }
    }
}
impl Vector2<u32>{
    pub fn to_float(self) -> Vector2<f32>{
        Vector2 { x: self.x as f32, y: self.y as f32 }
    }
    pub fn to_i32(self)-> Vector2<i32>{
        Vector2 { x: self.x as i32, y: self.y as i32 }
    }
}

impl<T: Into<f32>> Vector2<T>{
    pub fn length(self) -> f32{
        return (self.x.into().powf(2.) + self.y.into().powf(2.)).sqrt();
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.x)
         .field(&self.y)
         .finish()
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for &Vector2<T>{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x + rhs.x,self.y + rhs.y)
    }
}
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Vector2<T>{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x + rhs.x,self.y + rhs.y)
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign for Vector2<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for &Vector2<T>{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x - rhs.x,self.y - rhs.y)
    }
}
impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for Vector2<T>{
    type Output = Vector2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::<T>::new(self.x - rhs.x,self.y - rhs.y)
    }
}

impl<T: std::ops::SubAssign> std::ops::SubAssign for Vector2<T>{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vector2<T>{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::<T>::new(self.x*rhs,self.y*rhs)
    }
}
impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Vector2<T>{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::<T>::new(self.x/rhs,self.y/rhs)
    }
}

impl<T: std::ops::Neg<Output = T> + Copy> std::ops::Neg for &Vector2<T>{
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2::<T>::new(-self.x,-self.y)
    }
}

impl<T> std::ops::Index<i32> for Vector2<T>{
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 Index out of bounds")
        }
    }
}

impl<T> std::ops::IndexMut<i32> for Vector2<T>{

    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 Index out of bounds")
        }
    }
}

impl<T: std::ops::Rem<Output=T> + Clone> std::ops::Rem<T> for Vector2<T>{
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Vector2{x: self.x % rhs.clone(), y: self.y % rhs}
    }
}
impl<T: std::ops::RemAssign + Clone> std::ops::RemAssign<T> for Vector2<T>{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs.clone();
        self.y %= rhs;
    }
}


impl<T: Clone> From<[T;2]> for Vector2<T>{
    fn from(data: [T;2]) -> Self {
        Vector2 { x: data[0].clone(), y: data[1].clone() }
    }
}

impl From<Vector2<i32>> for Vector2<f32>{
    fn from(data: Vector2<i32>) -> Self {
        Vector2 { x: data.x as f32, y: data.y as f32 }
    }
}
impl From<Vector2<f32>> for Vector2<i32>{
    fn from(data: Vector2<f32>) -> Self {
        Vector2 { x: data.x as i32, y: data.y as i32 }
    }
}


impl<T: Default> Default for Vector2<T>{
    fn default() -> Self {
        Self { x: T::default(), y: T::default() }
    }
}