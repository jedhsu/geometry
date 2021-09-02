pub struct Cat {
    name: CategoryName,
}

pub trait Category<A>: Aleph
where
    A: CatName,
{
    type Cat;
}

pub type CategoryName = &str;

pub trait Subcategory<C>
where
    C: Category,
{
    fn is_subcategory_of(&self, cat: &Cat) -> bool;
    fn is_full_subcategory_of(&self, cat: &Cat) -> bool;
}

pub type Level = usize;

/// An aleph is an (∞, n)-category.
///
pub trait Aleph<N>
where
    N: Level,
{
}
