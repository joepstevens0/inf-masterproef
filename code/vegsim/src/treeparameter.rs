


#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum DistributionMode{
    BorchertHonda,
    PriorityList,
    None
}

impl std::fmt::Display for DistributionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceDividingMode {
    Markers,
    ShadowVoxels,
    None
}

impl std::fmt::Display for SpaceDividingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum GeneticParameter{
    BorchertHondaLambda(f32),
    BorchertHondaAlpha(f32),
    PoleLength(f32),
    AuxShootReq(f32)
}

#[derive(Debug, Clone)]
pub enum TreeParameter{
    Genetic(GeneticParameter),
    ResourceDistributionMode(DistributionMode),
    SpaceDividingMode(SpaceDividingMode),
    PruneModOn(bool)
}