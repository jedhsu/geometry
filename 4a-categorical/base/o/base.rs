// Toposic spaces are cateogires.
//
// Blurb on cpu tri.

use geometry::toposic::x::{Arrow, Thing};

pub trait Categorical<X, T>
where
    X: Thing,
    T: Arrow,
{
}

mod functor {
    // pub struct
}
