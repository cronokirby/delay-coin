use num_bigint::BigUint;

/// This represents a binary quadratic form.
///
/// This is equivalent to an element of a group of unknown order, and is used
/// to implement a verifiable delay function.
///
/// Operations on QForm are not constant-time, but this is fine within our usage
/// of verifiable delay functions.
///
/// Operations assume that the forms they're operating on have the same discriminant.
#[derive(Clone, Debug)]
struct QForm {
    a: BigUint,
    b: BigUint,
    c: BigUint,
}
