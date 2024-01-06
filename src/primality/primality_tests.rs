use rug::integer::SmallInteger;

use super::*;

#[test]
fn test_fermats_test() {
    let a = BigInt::from(2);

    // Special case test for n == 1.
    assert!(!fermats_test(&SmallInteger::from(1), a.clone()));

    for p in FIRST_100_PRIMES {
        let n = BigInt::from(*p);
        assert!(fermats_test(&n, a.clone()));
    }

    // Composites that are not Carmichael.
    for c in [2 * 3, 3 * 7, 2 * 11, 11 * 18, 53 * 59] {
        let n = BigInt::from(c);
        assert!(!fermats_test(&n, a.clone()));
    }

    // These should pass. Test may be somewhat meaningless but if they don't, something is surely
    // amiss.
    for carmichael in [561, 41041, 825265] {
        let n = BigInt::from(carmichael);
        assert!(fermats_test(&n, a.clone()));
    }
}

#[test]
#[should_panic]
fn test_fermats_test_zero_base() {
    let n = BigInt::from(3);
    let a = BigInt::from(0);
    fermats_test(&n, a);
}

#[test]
#[should_panic]
fn test_fermats_test_zero_n() {
    let n = BigInt::from(0);
    let a = BigInt::from(2);
    fermats_test(&n, a);
}

#[test]
#[should_panic]
fn test_fermats_test_negative_n() {
    let n = BigInt::from(-3);
    let a = BigInt::from(2);
    fermats_test(&n, a);
}

#[test]
fn test_miller_rabin_test() {
    let a = BigInt::from(2);

    // Special case test for n == 1.
    assert!(!miller_rabin_test(&SmallInteger::from(1), a.clone()));
    // Special case test for n == 2.
    assert!(miller_rabin_test(&SmallInteger::from(2), a.clone()));

    for p in FIRST_100_PRIMES {
        let n = BigInt::from(*p);
        assert!(miller_rabin_test(&n, a.clone()));
    }

    // Composites that are not Carmichael.
    for c in [2 * 3, 3 * 7, 2 * 11, 11 * 18, 53 * 59] {
        let n = BigInt::from(c);
        assert!(!miller_rabin_test(&n, a.clone()));
    }

    // These should not pass. 2 is a Miller-Rabin witness for these numbers.
    for carmichael in [561, 41041, 825265] {
        let n = BigInt::from(carmichael);
        assert!(!miller_rabin_test(&n, a.clone()));
    }

    // TODO: Test some pseudoprimes
}

#[test]
#[should_panic]
fn test_miller_rabin_test_zero_base() {
    let n = BigInt::from(3);
    let a = BigInt::from(0);
    miller_rabin_test(&n, a);
}

#[test]
#[should_panic]
fn test_miller_rabin_test_zero_n() {
    let n = BigInt::from(0);
    let a = BigInt::from(2);
    miller_rabin_test(&n, a);
}

#[test]
#[should_panic]
fn test_miller_rabin_test_negative_n() {
    let n = BigInt::from(-3);
    let a = BigInt::from(2);
    miller_rabin_test(&n, a);
}
