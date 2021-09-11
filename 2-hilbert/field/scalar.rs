/// Can think of scalar field as generalized notion of a directional derivative.
pub trait ScalarField<T>: Fn<Space<T>, Real> + Derivation {
    // fn inner_product(left: &T, right: &T) {
    //     Sum(left.Index);
    // }
}
