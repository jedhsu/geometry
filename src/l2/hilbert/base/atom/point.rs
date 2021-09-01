//! A point in a Hilbert space.

use geometry::hilbert::o::Dimension;

pub trait X<N, D>
where
    N: Dimension,
    D: Datatype,
{
    fn new(coordinates: Coordinates) {}
}

/// A point in a one-dimensional Hilbert space.
pub mod x1 {
    pub struct X1<A>
    where
        A: Dimension,
    {
        a: A,
    }

    impl<A> X1<A> {
        fn new(a: A) -> X1 {
            X1 { a }
        }
    }
}

/// A point in a two-dimensional Hilbert space.
pub mod x2 {
    pub struct X2<A, B> {
        a: Axel<Size>,
        b: Axel<Size>,
    }

    impl X2<Axel, Size> {
        fn new(a: Axel<Size>, b: Axel<Size>) -> X1 {
            X2 { a, b }
        }
    }
}

/// A point in a three-dimensional Hilbert space.
pub struct X3<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
    c: Axel<Size>,
}
