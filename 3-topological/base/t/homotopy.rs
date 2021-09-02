pub struct Homotopy<F, G>
where
    F: Continuous + Topological + Function,
    G: Continuous + Topological + Function,
{
    left: F,
    right: G,
}

pub trait Homotopic<S>
where
    S: Topological,
{
    fn is_homotopic_to(&self) -> bool;
}
