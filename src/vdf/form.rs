use num_bigint::BigInt;
use num_traits::sign::Signed;

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
    a: BigInt,
    b: BigInt,
    c: BigInt,
}

impl QForm {
    /// This creates the generator of a class group, with a given discriminant.
    ///
    /// The discriminant is expected to be the negation of a prime = 7 mod 8.
    pub fn generator(discriminant: &BigInt) -> QForm {
        debug_assert!(discriminant.is_negative(), "discriminator should be negative");
        // The absolute value is 7 mod 8, but discriminator is negative, hence 1 mod 8.
        debug_assert!(discriminant % 8 == 1.into(), "discriminator should be 7 mod 8");
        let a = BigInt::from(2u64);
        let b = BigInt::from(1u32);
        let c = -(discriminant - 1u64) / (&a << 2);
        return QForm { a, b, c };
    }
}
