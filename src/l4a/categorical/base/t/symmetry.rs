//! A symmetry at the categorical perspective is a groupoid.

pub trait Isometry<C>
where
    C: Category,
{
}

pub trait Symmetry<C>: Collection<Isometry<C>>
where
    C: Category,
{
}
