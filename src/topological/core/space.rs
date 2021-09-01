pub trait Topological: S
where
    S: Homeomorphism + Homotopy + Closeness,
{
}
pub trait Point<Identity> {}
