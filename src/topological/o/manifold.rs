use crate::topological::_o::dimension::Dimension;
use crate::topological::_o::space::Topological;

pub trait Manifold<_D>: Topological
where
    _D: Dimension,
{
    fn is_manifold(&self) -> bool;
}