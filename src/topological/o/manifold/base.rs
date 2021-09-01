use crate::topological::_o::dimension::Dimension;
use crate::topological::_o::space::Topological;

pub trait Manifold<N>: Topological
where
    N: Dimensions,
{
    fn is_manifold(&self) -> bool;
}
