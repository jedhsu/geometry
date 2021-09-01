use space::intuitionistic::core::point::Point;
use space::topological::core::set::Set;

pub trait Pointing<X>
where
    X: Element,
{
    fn is_pointing_to(&self) -> bool;
    fn pointing_to(&self) -> Set<Element>;
}
