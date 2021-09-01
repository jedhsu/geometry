pub trait Euclidean<_D>
where
    _D: Dimension,
{
}

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

pub type Axis = usize;
