pub type Dim = usize;

/// Goal is a rigorous definition of dimension that agrees with the triad and the i-1 topos.
///
/// NOTE - The any finite parameters can be representable by a single parameter fails beyond the point-set level,
/// because we are working with infinity levels.
///
/// It is quite clear that fractal dimension (and thus dimension) is missing a rigorous definition that aligns with
/// intuition.
///
/// TODO: Try and model fractal algorithm.

pub trait Dimension {
    fn dimension(&self) -> Dim;
}
