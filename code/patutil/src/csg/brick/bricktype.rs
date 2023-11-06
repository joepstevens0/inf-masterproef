use std::{str::FromStr, fmt};



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrickType {
    Sphere = 2,
    Box = 1,
    Layer = 0,
    Cylinder = 3,
    Cone = 4,
    Torus = 5
}

impl FromStr for BrickType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Sphere" => Ok(BrickType::Sphere),
            "Box" => Ok(BrickType::Box),
            "Layer" => Ok(BrickType::Layer),
            "Cylinder" => Ok(BrickType::Cylinder),
            "Cone" => Ok(BrickType::Cone),
            "Torus" => Ok(BrickType::Torus),
            _ => Err(()),
        }
    }
}

impl Default for BrickType {
    fn default() -> Self {
        Self::Sphere
    }
}
impl fmt::Display for BrickType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}