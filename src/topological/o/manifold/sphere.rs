use geometry::topological::o::{Dimension, Manifold};

pub trait Sphere<_D>: Manifold
where
    _D: Dimension,
{
}
