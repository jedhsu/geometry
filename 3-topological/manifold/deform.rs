pub trait Diffeomorphism<S, T>: Fn<S<g>, H<h>>
where
    S: Manifold,
    T: Manifold + Riemannian<h>,
{
    fn pushforward() -> {
        /// page 54 of the geodl book
        |x: S::Element| Differential<Self>, 
    }
}
