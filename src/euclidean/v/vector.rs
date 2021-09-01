//! Vectors in Euclidean space.

pub trait Vector<N, D> {
    fn add(&self, other: Vector<N, D>) -> Vector<N, D>;
    fn subtract(&self, other: Vector<N, D>) -> Vector<N, D>;

    fn multiply(&self, other: Vector<N, D>) -> Vector<N, D>;
    fn divide(&self, other: Vector<N, D>) -> Vector<N, D>;
}

/// A vector in a one-dimensional Euclidean space.
pub mod v1 {
    pub struct V1<Axel, Size> {
        a: Axel<Size>,
    }

    impl V1<Axel, Size> {
        fn new(a: Axel<Size>) -> V1 {
            V1 { a }
        }
    }
}

/// A vector in a two-dimensional Euclidean space.
pub mod v2 {
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

/// A vector in a three-dimensional Euclidean space.
pub struct X3<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
    c: Axel<Size>,
}
