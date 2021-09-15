/// Every view type is replaced with the type of self.
///
/// Seeks to be a generalization of Grothendieck's relative view.
use num::traits::Unsigned as Natural

pub trait Category<N>
where
    N: Natural,
{
    // type Object;
    // type Morphism;

    fn level(&self) -> Natural;
}

impl Category<N> {
    fn level(&self) -> Natural {
        N
    }
}

pub trait RelativeView<C>
where
    C: Category,
{

}
