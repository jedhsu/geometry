//! Euclidean space.

use geometry::hilbert::o::Hilbert;

pub trait Euclidean<D>: Hilbert
where
    D: Dimension,
{
}

pub type Loc<T> = T;

pub trait Coordinate<D>
where
    D: Dimension,
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
