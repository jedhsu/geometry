/// Can think of scalar field as generalized notion of a directional derivative.
pub trait VectorField<T>: Fn<Space<T>, V<Space<T>> + Derivation {
    fn inner_product(left: &T, right: &T) {
        Sum(left.Index);
    }
}
