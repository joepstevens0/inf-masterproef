use std::f32::consts::PI;
use crate::treeparameter::SpaceDividingMode;
use crate::treeparameter::DistributionMode;
use patutil::Vecf3;

pub const SEED: u64 = 50365756705;                      // seed used for everything random

pub const RESOURCE_DISTRIBUTION_MODE: DistributionMode = DistributionMode::BorchertHonda;   // change between BorchertHonda and PriorityList for resource distribution
pub const BORCHERT_HONDA_LAMBDA: f32 =  0.52;           // resource distribution ratio between auxillary bud and terminal when using BH-model
pub const BORCHERT_HONDA_ALPHA: f32 = 2.;               // for light to resources conversion 

pub const POLE_LENGTH: f32 = 1.;                        // length of vertical supportpole
pub const METAMER_BASE_LENGTH: f32 = 0.3f32;            // standard length of a metamer, value is multiplied by resources per growth it for real length

pub const AUX_SHOOT_REQUIREMENT: f32 = 1.8;             // amount of resources needed for a auxillary bud to create a metamer
pub const TERM_SHOOT_REQUIREMENT: f32 = 1.0;            // amount of resources needed for a terminal bud to create a metamer

pub const BUD_PERCEPTION_ANGLE: f32 = PI / 2.;          // angle a bud can see markers
pub const BUD_PERCEPTION_RADIUS_FACTOR: f32 = 1.1;      // radius for a bud to see markers
pub const OCCUPANCY_RADIUS_FACTOR: f32 = 1.0;           // radius a bud removes markers

pub const AXILLARY_PERTURBATION_ANGLE: f32 = PI / 38.0; // max angle between terminal bud and auxillary
pub const OPTIMAL_GROWTH_DIRECTION_WEIGHT: f32 = 0.2;   // weigth of the optimal growth direction used when choosing metamer direction
pub const SHED_TRESHHOLD: f32 = 0.01;                   // min resources a branch needs before it is shed

pub const TROPISM_START_WEIGTH: f32 = 0.1;              // starting weight for tropism vector on when choosing metamer direction
pub const TROPISM_DIR: Vecf3 = Vecf3::new(0., -0., 0.); // direction of tropism
pub const TROPISM_CHANGE_RATE: f32 = 1.01;              // rate tropism weight changes every iteration

pub const BOUNDING_BOX_SIDE: f32 = 50.;                 // length of a bounding box side

pub const SPACE_DIV_MODE: SpaceDividingMode = SpaceDividingMode::ShadowVoxels;   // change between shadowvoxels and markers for light and optimal growth direction calculation
pub const SPACE_DIV_RESOLUTION: u32 = 100;              // resolution for shadowvoxels and markers (value of 100 = 100*100*100 voxels and markers divided evenly in bounding box)

// shadowvoxel parameters
pub const SHADOW_VOXEL_A: f32 = 0.1;
pub const SHADOW_VOXEL_B: f32 = 1.5;
pub const SHADOW_VOXEL_C: f32 = 1.;
pub const SHADOW_VOXEL_MAX_SHADOW: f32 = 5.;
pub const SHADOW_VOXEL_PIRAMID_LAYERS: i32 = 5;

pub const WIDTH_GROW_EXPONENT: f32 = 1.9f32;            // change in width depending on metamers attached
pub const WIDTH_MIN_VALUE: f32 = 1.0e-8f32;             // minimum width a metamer can have

pub const BUD_RECOVERY_SPEED: f32 = 0.;                 // recovery speed a bud repairs from damge due to pruning, damage starts at 1, recovery speed default 0: bud wil never recover