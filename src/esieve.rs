//! Sieve of Erasthotenes -- Find all the primes up to a and including given bound.
//!
//!

use fixedbitset::FixedBitSet;

#[cfg(test)]
mod esieve_tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    BadBound,
    BadMemory,
}

pub type Result<T> = std::result::Result<T, Error>;

///
/// The state of the Erastothenes Sieve after some number of iterations (or none).
///
#[derive(Debug)]
pub struct SieveState {
    // This is a map n-1 -> bool, telling us whether we know that n is composite or not.
    // We accept that this mapping wastes one bit, as 1 is never prime.
    is_known_composite: FixedBitSet,
    primes: Vec<usize>,
    upper_bound: usize,
    last_divisor: usize,
}

impl SieveState {
    /// Initialize the state with upper bound `n`. Only primes up to and including this bound will
    /// be generated.
    ///
    /// Note that this might allocate a fair chunk of memory.
    ///
    /// **Complexity:** `O(n)` in time and `O(n)` in memory.
    pub fn with_upper_bound(n: usize) -> Result<SieveState> {
        if n <= 0 {
            Err(Error::BadBound)
        } else {
            let is_known_composite = FixedBitSet::with_capacity(n);
            Ok(SieveState {
                is_known_composite,
                primes: vec![],
                upper_bound: n,
                last_divisor: 1,
            })
        }
    }

    pub fn primes_found(&self) -> &[usize] {
        &self.primes
    }
}

pub enum EndCondition {
    UpperBoundReached,
}

impl Default for EndCondition {
    /// Runs the sieve until the upper bound has been reached.
    fn default() -> Self {
        Self::UpperBoundReached
    }
}

fn find_next_divisor(
    last_divisor: usize,
    upper_bound: usize,
    is_known_composite: &FixedBitSet,
) -> Option<usize> {
    let mut potential_divisor = last_divisor;
    while potential_divisor < upper_bound {
        /*
            Note that this step should not be able to overflow:
            Assume upper_bound = USIZE::max(), and adding 1 to potential_divisor would overflow.
            Then potential_divisor = USIZE::max().
            But USIZE::max() is not less than itself.
        */
        potential_divisor += 1;

        if !is_known_composite[potential_divisor - 1] {
            return Some(potential_divisor);
        }
    }
    None
}

fn mark_multiples_as_composite(
    divisor: usize,
    upper_bound: usize,
    is_known_composite: &mut FixedBitSet,
) {
    let mut last_multiple = divisor;
    loop {
        if let Some(multiple) = last_multiple.checked_add(divisor) {
            if multiple <= upper_bound {
                is_known_composite.set(multiple - 1, true);
                last_multiple = multiple;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn sieve_once(state: &mut SieveState) -> Result<bool> {
    match find_next_divisor(
        state.last_divisor,
        state.upper_bound,
        &state.is_known_composite,
    ) {
        Some(divisor) => {
            // Assuming the previous steps have been correct, divisor should be prime:
            state.primes.push(divisor);

            mark_multiples_as_composite(divisor, state.upper_bound, &mut state.is_known_composite);
            state.last_divisor = divisor;

            Ok(true)
        }
        None => Ok(false),
    }
}

///
/// Sieves until some condition is met. Usually this ends when all the primes have been found.
///
pub fn run(state: &mut SieveState, stop_when: EndCondition) -> Result<()> {
    use EndCondition::*;
    match stop_when {
        UpperBoundReached => while sieve_once(state)? {},
    }
    Ok(())
}
