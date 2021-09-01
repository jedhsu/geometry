use space::intuitionistic::element::Element;

pub trait Point<X>: Element {
    type Identity;
}

pub struct Identity {
    parts: Vec<Symbol>,
}
