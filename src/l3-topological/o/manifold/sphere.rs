use geometry::topological::o::{Dimension, Manifold};

pub trait Sphere<N>: Manifold
where
    N: Dimension,
{
    n_dimensions: usize = N;
    // fn n_dimensions(&self) -> usize;
}

pub struct Sphere1 {}

impl Sphere<N> for Sphere1 {}
pub struct Sphere2 {}
pub struct Sphere3 {}
