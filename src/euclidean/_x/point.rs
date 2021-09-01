pub trait X<_D>
where
    _D: Dimension,
{
    fn new(coordinates: Coordinates) {}
}

pub struct X1<Axel, Size> {
    a: Axel<Size>,
}

pub struct X2<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
}

pub struct X3<Axel, Size> {
    a: Axel<Size>,
    b: Axel<Size>,
}
