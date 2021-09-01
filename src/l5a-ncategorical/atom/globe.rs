use geometry::triadic::atom::shape;

/// Note that the datatype parameter D is now shape.
pub trait Globe<N, D>: Shape
where
    N: Dimension,
    D: Shape,
{
}

pub trait Globular {}

pub struct Globe1<D> {}

pub struct Globe2<D> {}
