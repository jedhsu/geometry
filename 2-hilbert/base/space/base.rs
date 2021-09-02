//! Hilbert space.

use geometry::topological::o::Topological;

pub trait Hilbert<N, D>: Topological
where
    N: Dimension,
    D: Datatype,
{
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

