use crate::topological::_t::{Closeness, Name};
use crate::topological::_x::{Homeomorphism, Homotopy};

pub type Dim = usize;

/// Hausdorff dimension, a topologically rigorous definition of topological dimension.
///
/// Intuitive concept of dimension of a geometric object is the number of independent parameters
/// one needs to pick out a unique point. Note how this is arity!
///
/// NOTE - The any finite parameters can be representable by a single parameter fails beyond the point-set level,
/// because we are working with infinity levels.

pub trait Dimension {
    fn dimension(&self) -> Dim;
}

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
