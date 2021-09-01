use std::ops::Index;

pub trait Sequence<X>
where
    X: Point,
{
}

impl Index<usize> for S
where
    S: Topological,
{
    fn index(&self, ordinal: i32) -> &Self::Point {}
}
