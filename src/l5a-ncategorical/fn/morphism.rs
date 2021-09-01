//! A morphism at the polycategorical level is a [k-morphism][kmorph]
//1
//! [kmorph][https://ncatlab.org/nlab/show/k-morphism]

pub trait Morphism<N, D>
where
    N: Dimension,
    D: Category,
{
}

pub struct Morphism1 {}

pub struct Morphism 2
