use maths::Vector3;
use models::assembly::thesis::{
    ThesisChamber, ThesisDetectorAlpha, ThesisDetectorBeta, ThesisHolderCircle, ThesisStage,
};
use models::parts::chamber::Chamber;
use models::parts::equipment::Equipment;
use models::parts::holder::Holder;
use models::parts::stage::Stage;
use models::position::sixaxis::SixAxis;
use paths::path::PathResult;
use paths::resolver::stage::down_rotate_find::DownRotateFindResolver;
use paths::resolver::stage::StagePathResolver;
use paths::timed;

const START_POSITION: SixAxis = SixAxis {
    pos: Vector3::new(0e-3, 0e-3, 55e-3),
    rot: Vector3::new(0_f64.to_radians(), 0_f64.to_radians(), 0_f64.to_radians()),
};
const END_POSITION: SixAxis = SixAxis {
    pos: Vector3::new(-55e-3, 0e-3, 55e-3),
    rot: Vector3::new(0_f64.to_radians(), 40_f64.to_radians(), 0_f64.to_radians()),
};

#[test]
fn entrypoint() {
    for _ in 0..100 {
        find_path();
    }
}

fn find_path() {
    let chamber = setup_chamber();
    let equipment = setup_equpment();
    let immovable = chamber
        .full()
        .extended(equipment[0].collider())
        .extended(equipment[1].collider());

    let mut stage = setup_stage();
    let holder = setup_holder();
    stage.swap_holder(Some(holder));

    let mut stage_resolver = setup_stage_resolver();
    let (_, update_time) = timed!({
        stage_resolver
            .update_state(&START_POSITION, stage.as_ref(), &immovable)
            .unwrap();
    });
    println!("State updated in {:?} ms", update_time.as_millis());

    let (path, path_time) = timed!({
        stage_resolver.resolve_path(&START_POSITION, &END_POSITION, stage.as_ref(), &immovable)
    });
    println!("Path found in {:?} ms", path_time.as_millis());

    assert!(matches!(path, PathResult::Path(_)));
}

fn setup_chamber() -> Box<dyn Chamber> {
    Box::new(ThesisChamber::default())
}

fn setup_equpment() -> Box<[Box<dyn Equipment>]> {
    Box::new([
        Box::new(ThesisDetectorAlpha::default()),
        Box::new(ThesisDetectorBeta::default()),
    ])
}

fn setup_stage() -> Box<dyn Stage> {
    Box::new(ThesisStage::default())
}

fn setup_holder() -> Box<dyn Holder> {
    Box::new(ThesisHolderCircle::default())
}

fn setup_stage_resolver() -> Box<dyn StagePathResolver> {
    Box::new(DownRotateFindResolver::new(
        Vector3::ZERO,
        SixAxis {
            pos: Vector3::new(1.0, 1.0, 1.0),
            rot: Vector3::new(1_f64.to_radians(), 1_f64.to_radians(), 1_f64.to_radians()),
        },
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(-0.135, -0.125, -0.125),
        Vector3::new(0.125, 0.125, 0.125),
        Vector3::new(0.01, 0.01, 0.01),
        Vector3::new(0.006, 0.006, 0.006),
        Vector3::new(0.001, 0.001, 0.001),
        SixAxis {
            pos: Vector3::new(1.0, 1.0, 1.0),
            rot: Vector3::new(1_f64.to_radians(), 1_f64.to_radians(), 1_f64.to_radians()),
        },
    ))
}
