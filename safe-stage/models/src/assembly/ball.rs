use crate::position::sixaxis::SixAxis;
use maths::Vector3;

mod chamber;
mod stage;

pub use chamber::ball_chamber_triangles;
pub use chamber::BallChamber;
pub use stage::ball_stage_triangles;
pub use stage::BallStage;

pub const LIMITS: (SixAxis, SixAxis) = (
    SixAxis {
        pos: Vector3::new(-1450e-3, -1150e-3, 0.0),
        rot: Vector3::ZERO,
    },
    SixAxis {
        pos: Vector3::new(550e-3, 550e-3, 0.0),
        rot: Vector3::ZERO,
    },
);

const ONE_DEGREE: f64 = 0.0174532925;
pub const SPEED: SixAxis = SixAxis {
    pos: Vector3::new(1.0, 1.0, 1.0),
    rot: Vector3::new(ONE_DEGREE, ONE_DEGREE, ONE_DEGREE),
};

pub const START_POSITION: Vector3 = Vector3::ZERO;
pub const END_POSITION: Vector3 = Vector3::new(-1250e-3, -950e-3, 0.0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_data() {
        let _chamber = BallChamber::default();
        let _stage = BallStage::default();
    }
}
