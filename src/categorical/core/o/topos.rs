use geometry::topological::Topological;

pub trait Toposic<O>: Category + Set
where
    O: Topological,
{
}

pub trait Morphism {}

/// Geometric Morphism
///
/// Morph from the perspective of the generalized topological space.
pub trait Transform<_O_, _P_>: Morphism
where
    _O_: Topos,
    _P_: Topos,
{
}

/// Analytic Morphism
///
/// Morph from the perspective of the internal logic of the structure.
pub trait Transmute<_O_: Topos>: Morphism {}

pub trait Grothendieck<O>: Toposic<O>
where
    O: Topological,
{
}
/// Geometrization is the [embedding of a geometry][geometric-embedding] onto a category.
///
/// [geometric-embedding][https://ncatlab.org/nlab/show/geometric+embedding]
pub trait Geometrization: Transform {}
