//! A Hilbert space is a function space (elements are functions) with vector space structure.

use geometry::topological::o::Topological;

pub trait Hilbert<N, D>: Topological
where
    N: Dimension,
    D: Channel,
{
    pub type Channel;
    pub type Set<D>

}

pub enum Operator {
    Addition,
    Multiplication,
    InnerProduct
}

pub struct Element {
    operator: Operator,
    left: Box<Element>,
    right: Box<Element>,
}

/// Addition, scalar multiplication, and inner product are defined on the elements of Hilbert
/// spaces.
pub trait Channel {
    /// Structure of closed under addition.
    fn add(&self, other: Channel) -> Self {

    }
}

pub type Loc<T> = T;

pub trait Coordinate<_D>
where
    _D: Dimension,
{
    fn new(locs: Vec<Loc<T>>);
}

mod interval {
    pub struct Interval<D>
    where
        D: Datatype,
    {
        left: D,
        right: D,
    }

    impl<D> Interval<D> {
        fn new(bounds: (D, D)) -> Interval<D> {
            Interval { bounds[0], bounds[1] }
        }
    }

}

mod axis {
    pub struct Axis = usize;
}

