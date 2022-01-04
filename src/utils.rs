use num_traits::PrimInt;

/**
 * Compute base^exp mod m
 *
 * Adapted from: https://github.com/rust-num/num-traits/blob/master/src/pow.rs
 */
pub fn pow_mod<T>(mut base: T, mut exp: usize, m: T) -> T
where
    T: PrimInt,
{
    if exp == 0 {
        return T::one();
    }

    while exp & 1 == 0 {
        base = (base * base) % m;
        exp >>= 1;
    }
    if exp == 1 {
        return base;
    }

    let mut acc = base;
    while exp > 1 {
        exp >>= 1;
        base = (base * base) % m;
        if exp & 1 == 1 {
            acc = (acc * base) % m;
        }
    }
    acc
}
