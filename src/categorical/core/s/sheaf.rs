pub trait Presheaf<C>: Functor<Mirror<O>, Set> + Category<A>
where
    C: Category,
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

pub trait Site<J: Coverage>: Category<C>
where
    O: Category,
{
}

pub trait Sheaf<CJ>: Presheaf<C>
where
    CJ: Site<J>,
    J: Coverage<C>,
    C: Category,
{
}
