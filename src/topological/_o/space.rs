pub trait Basis {}

pub trait Topological<O, X>: Toposic
where
    O: Homeomorphism + Homotopy,
    X: Named + Closeness,
{
}

pub trait Spaces<O>
where
    O: Topological,
{
}
