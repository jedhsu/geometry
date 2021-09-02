// pub trait Functor<O, P> {
//     type O;
//     type P;

//     fn transmute(&self, x: Element);

//     mod _transmute {
//         fn _transmute_thing(&self, x: Thing);
//         fn _transmute_arrow(&self, x: Arrow);
//     }
// }

pub struct Adjoint<C, D>
where
    C: Category,
    D: Category,
{
    left: C,
    right: D,
}

pub trait Isomorphism: Morphism {}

// Natural Isomorphism is named Equimorphism.
//
pub trait Equimorphism: Isomorphism {}
