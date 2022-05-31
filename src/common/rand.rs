use std::num::Wrapping;
use std::ops::Range;

const GMWC_MINUSA0: Wrapping<u64> = Wrapping(0x7d084a4d80885fu64);
const GMWC_A0INV: Wrapping<u64> = Wrapping(0x9b1eea3792a42c61);
const GMWC_A1: Wrapping<u64> = Wrapping(0xff002aae7d81a646);

/// implementation from https://en.wikipedia.org/wiki/Multiply-with-carry_pseudorandom_number_generator
pub struct Random {
    x: Wrapping<u64>,
    c: Wrapping<u64>,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self {
            x: Wrapping(0),
            c: Wrapping(seed as u64 + 1), // note that this c always admits to the state condition
        }
    }

    pub fn next(&mut self) -> u64 {
        let t: Wrapping<u128> = Wrapping(GMWC_A1.0 as u128 * self.x.0 as u128 + self.c.0 as u128);
        self.x = GMWC_A0INV * Wrapping(t.0 as u64);
        self.c = Wrapping(
            ((t + Wrapping(GMWC_MINUSA0.0 as u128) * Wrapping(self.x.0 as u128)) >> 64).0 as u64,
        );
        self.x.0
    }

    /// calculates a pseudo random number inside the specified range, excluding the end-value
    /// e.g. `range(Range { start: 3, end: 5 }` always returns 3 or 4
    pub fn range(&mut self, r: Range<usize>) -> usize {
        self.next() as usize % (r.end - r.start) + r.start
    }
}
