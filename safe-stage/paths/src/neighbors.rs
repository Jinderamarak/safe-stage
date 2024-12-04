pub mod limited_rotation_grid;
pub mod no_rotation_grid;

pub trait NeighborStrategy<N> {
    fn neighbors(&self, current: &N) -> impl Iterator<Item = N> + '_;
}
