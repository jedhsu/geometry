//! The shape is the atomic construct of higher category theory.
//!
//! Note the self-reference.
pub trait Shape<N, D>
where
    N: Dimension,
    D: Shape,
{
}
