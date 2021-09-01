//! Euclidean n-cubes.
//!
//! THESE ARE ACTUALLY SLICES!

use geometry::euclidean::o::base::Interval;

pub trait Cube<N, D>: Euclidean
where
    N: Dimension,
    D: Datatype,
{
}

/// A segment, i.e. a cube in one-dimensional Euclidean space.
mod cube1 {
    pub struct Cube1<D>
    where
        D: Datatype,
    {
        a: Interval<D>,
    }

    // pub type Segment = Cube1

    impl<D> Cube1<D> {
        fn new(a: (D, D)) -> Cube1<D> {
            Cube1 {
                a: Interval::new(a),
            }
        }
    }
}

/// A square, i.e. a cube in two-dimensional Euclidean space.
mod cube2 {
    pub struct Cube2<D>
    where
        D: Datatype,
    {
        a: Interval<D>,
        b: Interval<D>,
    }

    impl<D> Cube2<D> {
        fn new(a: (D, D), b: (D, D)) -> Cube2<D> {
            Cube2 {
                a: Interval::new(a),
                b: Interval::new(b),
            }
        }
    }
}

/// A traditional cube, i.e. a cube in three-dimensional Euclidean space.
mod cube3 {
    pub struct Cube3<D>
    where
        D: Datatype,
    {
        a: Interval<D>,
        b: Interval<D>,
        c: Interval<D>,
    }

    impl<D> Cube4<D> {
        fn new(a: (D, D), b: (D, D), c: (D, D)) -> Cube3<D> {
            Cube2 {
                a: Interval::new(a),
                b: Interval::new(b),
                c: Interval::new(c),
            }
        }
    }
}

/// A tesseract, i.e. a cube in four-dimensional Euclidean space.
mod cube4 {
    pub struct Cube4<D>
    where
        D: Datatype,
    {
        a: Interval<D>,
        b: Interval<D>,
        c: Interval<D>,
    }

    impl<D> Cube4<D> {
        fn new(a: (D, _D), b: (D, D), c: (D, D), d: (D, D)) -> Cube4<D> {
            Cube2 {
                a: Interval::new(a),
                b: Interval::new(b),
                c: Interval::new(c),
                d: Interval::new(d),
            }
        }
    }
}
