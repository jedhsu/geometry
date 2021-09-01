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
