/// Can think of field as generalized notion of a directional derivative.
pub trait VectorField<T>: Fn<Space<T>, V<Space<T>> + Derivation {
    fn inner_product(left: &T, right: &T) {
        Sum(left.Index);
    }
}

_∇_ = Gradient
// The local direction of steepest increase of a scalar field on the manifold.
pub trait Gradient<VectorField<u>> {
}
