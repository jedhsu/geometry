pub struct Homeomorphism<X, Y>
where
    X: Topological,
    Y: Topological,
{
    left: X,
    right: Y,
}
