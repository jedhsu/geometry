use crate::topological::_t::{Closeness, Name};
use crate::topological::_x::{Homeomorphism, Homotopy};

pub type Dimension = usize;

pub trait Basis {}

pub trait Topological<O, X>: Toposic
where
    O: Homeomorphism + Homotopy,
    X: Name + Closeness,
{
}

pub trait Spaces<O>
where
    O: Topological,
{
}
