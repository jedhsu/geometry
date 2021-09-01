pub struct Cat {
    name: CategoryName,
}

pub trait Category<A>
where
    A: CatName,
{
    type Cat;
}

pub type CategoryName = &str;
