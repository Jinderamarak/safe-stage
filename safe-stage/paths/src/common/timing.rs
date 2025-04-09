#[macro_export]
macro_rules! timed {
    ($block:block) => {{
        let start = std::time::Instant::now();
        let result = { $block };
        (result, start.elapsed())
    }};
}

pub use timed;
