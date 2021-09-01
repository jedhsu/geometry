pub type Loc<T> = T;

pub trait Coordinate<_D>
where
    _D: Dimension,
{
    fn new(locs: Vec<Loc<T>>);
}
