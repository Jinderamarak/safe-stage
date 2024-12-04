use crate::position::sixaxis::SixAxis;
use maths::Vector3;
use std::f64::consts::{FRAC_PI_4, FRAC_PI_6, PI};

mod chamber;
mod equipment;
mod holders;
mod retract;
mod stage;

pub use chamber::ThesisChamber;
pub use equipment::*;
pub use holders::*;
pub use retract::ThesisRetract;
pub use stage::ThesisStage;

pub const LIMITS: (SixAxis, SixAxis) = (
    SixAxis {
        pos: Vector3::new(-135e-3, -125e-3, -125e-3),
        rot: Vector3::new(0.0, -FRAC_PI_4, -PI),
    },
    SixAxis {
        pos: Vector3::new(125e-3, 125e-3, 125e-3),
        rot: Vector3::new(0.0, FRAC_PI_6, PI),
    },
);

const ONE_DEGREE: f64 = 0.0174532925;
pub const SPEED: SixAxis = SixAxis {
    pos: Vector3::new(1.0, 1.0, 1.0),
    rot: Vector3::new(ONE_DEGREE, ONE_DEGREE, ONE_DEGREE),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_data() {
        let _chamber = ThesisChamber::default();
        let _retract = ThesisRetract::default();
        let _stage = ThesisStage::default();
        let _circle = ThesisHolderCircle::default();
        let _square = ThesisHolderSquare::default();
        let _alpha = ThesisDetectorAlpha::default();
        let _beta = ThesisDetectorBeta::default();
    }
}
