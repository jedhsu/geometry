use num::traits::Unsigned as Positive

pub trait MetricSpace: Category {
    fn distance(left: &Self::Element, right: &Right::Element) -> Positive;
}

pub trait Distance<X>: F where X: Set, F: Fn<A, B>, A: Set, B: Set {};


// pub trait Triangular: Predicate {
//     type left: &'static str = Distance<X, Y> + Distance<Y, X> >= Distance<X, X>
// }


