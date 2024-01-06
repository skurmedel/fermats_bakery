//! # Primality testing
//! Some reference implementations of primality testing methods. Since this is using `rug` (and
//! indirectly GMP) you'd likely be better off using the methods on [BigInt].
//!
//! Most of the tests found here are probalistic. Such tests do not give a definite answer as to
//! whether a given integer is prime or not. If we vary the inputs however, and the integer passes
//! each time, we can be more and more certain it is truly prime.
//!
//! Many of the simpler tests suffer from pseudoprimes. These are composite integers that pass a
//! certain test no matter what. The most famous of these are the Carmichael numbers, which will all
//! pass Fermat's test.
//!
//! One of the strongest tests as of 2023 is the Baillie-PSW test. It is a battery of several tests
//! and no composite number below 2**64 is known to pass. Currently there are no known pseudoprimes.

use std::{
    cell::OnceCell,
    num::NonZeroU32,
    ops::{Add, Div, ShrAssign},
};

#[cfg(test)]
mod primality_tests;

use rug::{integer::SmallInteger, ops::DivFrom};

use super::*;

/// Fermat's test for primality.
///
/// *This is a probalistic test: primes will always pass, but some composites may also pass. If a
/// number fails, it is guaranteeed to be composite.*
///
/// This test has a famous achilles heel; it will not work for Carmichael numbers: these are
/// composite numbers that always pass Fermat's test for all bases.
///
/// The test checks whether
/// ```text
///     a**n = a   mod n
/// ```
///
/// # Example
///
/// ```
/// use fermats_kitchen::primality::*;
/// assert!(fermats_test(&11.into(), 2.into()));
/// assert!(!fermats_test(&8.into(), 2.into()));
///
/// // The smallest Carmichael number 561 (it divisible by 3 and certainly not prime)
/// assert!(fermats_test(&561.into(), 2.into()));
/// assert!(fermats_test(&561.into(), 3.into()));
/// ```
///
/// # Arguments
/// - `n` is the integer we wish to test.
/// - `a` is the base, a commonly used value is 2. `a` should preferably be larger than 1 and less
///   than `n - 1`. (`a=1` gives no info, and neither `a=n-1` nor `n | a`)
///
/// # Panics
/// - `a == 0`
/// - `n < 1`
pub fn fermats_test(n: &BigInt, a: BigInt) -> bool {
    // If n is prime, the order of (Z/nZ)* is n - 1
    //  => a**n = a   mod n

    if n == &make_one() {
        return false;
    }

    assert!(!a.is_zero());
    assert!(n.is_positive());

    let mut c = a.clone();
    c.pow_mod_mut(n, n)
        .expect("n was negative and an inverse did not exist");
    c == a.modulo(n)
}

fn make_one() -> SmallInteger {
    SmallInteger::from(1)
}

fn make_two() -> SmallInteger {
    SmallInteger::from(2)
}

/// The Miller-Rabin primality test.
///
/// *This is a probalistic test: primes will always pass, but some composites may also pass. If a
/// number fails, it is guaranteed to be composite.*
///
/// Unlike Fermat's test, this is doesn't suffer from any Carmichael numbers, but it is decidedly
/// slower to run.
///
/// `a` is called a Miller-Rabin witness for a given `n` if `miller_rabin_test(n, a) == false`. For
/// any reasonably large integer `n`, about 75% of the integers less than `n` are witnesses. If `n`
/// fails several rounds of the Miller-Rabin test (with different `a`), it is likely prime.
///
/// Conversely, a given `a` is called a liar if `n` passes for `a`.
///
/// # Arguments
/// - `n` is the integer we wish to test.
/// - `a` is the base, a commonly used value is 2. `a` should preferably be larger than 1 and less
///   than `n - 1`. (`a=1` gives no info, and neither `a=n-1` nor `n | a`)
///
/// # Panics
/// - `b == 0`
/// - `n < 1`
pub fn miller_rabin_test(n: &BigInt, mut a: BigInt) -> bool {
    assert!(!a.is_zero());
    assert!(n.is_positive());

    // TODO: probably much quicker way to check this? `is_even`?
    if n == &make_two() {
        return true;
    }

    // Factor n - 1 == 2**k q
    let mut q = BigInt::from(n - 1);
    let mut k = BigInt::new();

    loop {
        if q > 1 && q.is_even() {
            k += 1;
            q.shr_assign(1);
        } else {
            break;
        }
    }

    // If n is prime:
    //      m = pi(n) = n - 1 => a**m = 1   mod n
    // and if n > 2,
    //      m even
    // and since
    //      x**2     = 1        mod n   has exactly two solutions
    // we have
    //      a**(m/2) = +/- 1    mod n
    //
    // Hence if n is actually prime,
    //  either one number number a**(2**i q) mod n (i=0,...,k-1) is -1 and we get 1 by squaring
    //  or a**(q) == 1
    let minus_one = BigInt::from(n - 1);

    a.pow_mod_mut(&q, &n).expect("Should have a result");

    if &a == &make_one() {
        return true;
    }

    let mut i = BigInt::from(0);
    while i < k {
        if a == minus_one {
            return true;
        }
        a.pow_mod_mut(&make_two(), &n)
            .expect("Should have a result");

        i += 1;
    }

    return false;
}

struct PrimalityTestOptions {
    pub rounds: NonZeroU32,
}

impl PrimalityTestOptions {
    pub fn suggested(a: &BigInt) -> Self {
        todo!()
    }
}

/// Uses a combination of Fermat's and Miller-Rabin to test whether an integer `a` is a likely
/// prime. If the test fails, the integer is guaranteed composite. If the test succeeds, it is with
/// high likelihood a prime.
fn probabilistic_primality_test(n: &BigInt) -> Primality {
    todo!()
}

pub enum Primality {
    Composite,
    ProbablyPrime,
    Prime,
}

pub const FIRST_100_PRIMES: &'static [u32] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
];
