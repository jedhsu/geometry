pub trait Seq<N, D>
where
    N: Dimension,
    D: Datatype,
{
}

pub trait Cauchy<N, D>: Seq<N, D>
where
    N: Dimension,
    D: Datatype,
{
    fn is_cauchy(&self) -> bool;
}
