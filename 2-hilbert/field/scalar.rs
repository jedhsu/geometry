pub trait ScalarField<T>: Fn<Space<T>, Real> + Derivation {
    // fn inner_product(left: &T, right: &T) {
    //     Sum(left.Index);
    // }
}
