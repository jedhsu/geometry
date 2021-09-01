pub trait X<_D>
where
    _D: Dimension,
{
    fn new(coordinates: Coordinates) {}
}

/// A point in one-dimensional Euclidean space.
pub struct X1<Axel, Size> {
    a: Axel<Size>,
}

impl X1<Axel, Size> {
    fn new(a: Axel<Size>) -> X1 {
        X1 { a }
    }
}

/// A point in two-dimensional Euclidean space.
pub struct X2<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
}

impl X2<Axel, Size> {
    fn new(a: Axel<Size>, b: Axel<Size>) -> X1 {
        X2 { a, b }
    }
}

/// A point in three-dimensional Euclidean space.
pub struct X3<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
    c: Axel<Size>,
}
