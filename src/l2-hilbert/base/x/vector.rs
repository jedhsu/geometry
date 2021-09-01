pub trait InnerProduct<V>
where
    V: Vector,
{
    fn inner_product(&self, other: Vector) -> Scalar;
}
