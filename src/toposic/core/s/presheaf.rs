pub trait Presheaf<O>: Functor<Mirror<O>, Set> + Category<A>
where
    O: Category,
    A: CategoryName,
{
    type Name;

    fn new(sc: SmCat) -> Presheaf<SmCat>;
}

pub trait Coverage<C>: Collection
where
    C: Category,
{
}

// pub trait Site<J>: Category<C>
// where
//     O: Category,
