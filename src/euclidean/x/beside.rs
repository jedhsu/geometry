use crate::euclidean::x::Point;

pub trait Beside<X>: Adjacent
where
    X: Point,
{
    fn is_beside_to(&self) -> bool;
    fn beside_to(&self) -> Set<Point>;
}
