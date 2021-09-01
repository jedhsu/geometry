pub trait Point<X>: Element {
    type Identity;

    fn is_near() -> bool;
    fn near() -> Set<Point>;
}

pub struct Identity {
    parts: Vec<Symbol>,
}
