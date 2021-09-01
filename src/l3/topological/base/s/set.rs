pub trait Set<X>
where
    X: Point,
{
    fn iterate(&self);
}

impl Iterator for Set<X> {
    type Item = X;
}
