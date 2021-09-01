use space::topological::core::point::Point;
use space::topological::core::set::Set;

pub trait Closeness<X>
where
    X: Point,
{
    fn is_close_to(&self) -> bool;
    fn close_to(&self) -> Set<Point>;
}
