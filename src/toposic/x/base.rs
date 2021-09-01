// An element of a toposic space.
//
// This is the most general, and can allow for self-referencing of it's own trait structure. TODO -
// clarify

use space::toposic::s::Collection;
use space::toposic::x::Thing;

use std::ops::Shr;

pub trait Thing {}

mod pointing {
    pub trait Pointing<X>
    where
        X: Thing,
    {
        fn is_pointing_to(&self, other: Thing) -> bool;
        fn points_to(&self) -> Collection<Thing>;
    }
}

mod identity {
    pub trait Identity<X>
    where
        X: Thing,
    {
        fn is_same_as(&self, other: Thing) -> bool;
    }
}

mod combine {
    pub trait Combine<X, Y>
    where
        X: Thing,
        Y: Thing,
    {
        fn combine(x: X, y: Y);
    }
}
