//! A point in a Euclidean space.

use geometry::hilbert;

pub trait X<N, D>: hilbert::X
where
    N: Dimension,
    D: Datatype,
{
    fn new(coordinates: Coordinates) {}
}

/// A point in a one-dimensional Euclidean space.
pub mod x1 {
    pub struct X1<Axel, Size> {
        a: Axel<Size>,
    }

    impl X1<Axel, Size> {
        fn new(a: Axel<Size>) -> X1 {
            X1 { a }
        }
    }
}

/// A point in a two-dimensional Euclidean space.
pub mod x2 {
    pub struct X2<Axel, Size> {
        a: Axel<Size>,
        b: Axel<Size>,
    }

    impl X2<Axel, Size> {
        fn new(a: Axel<Size>, b: Axel<Size>) -> X1 {
            X2 { a, b }
        }
    }
}

/// A point in a three-dimensional Euclidean space.
pub struct X3<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
    c: Axel<Size>,
}
