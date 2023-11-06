use std::{str::FromStr, fmt};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrickOp {
    Union = 0,
    Diff = 2,
    Inter = 1,
}

impl FromStr for BrickOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Union" => Ok(BrickOp::Union),
            "Diff" => Ok(BrickOp::Diff),
            "Inter" => Ok(BrickOp::Inter),
            _ => Err(()),
        }
    }
}
impl fmt::Display for BrickOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for BrickOp {
    fn default() -> Self {
        Self::Union
    }
}