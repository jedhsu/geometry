use geometry::topological::s::Set;
use geometry::topological::x::Point;

pub trait Closeness<X>
where
    X: Point,
{
    fn is_close_to(&self) -> bool;
    fn close_to(&self) -> Set<Point>;
}
