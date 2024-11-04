/// Represents the plaintext modulus
pub type PlaintextModulus = u64;

/// 64-bit multiply into a 128-bit result
#[inline]
pub fn mul_128(a: u64, b: u64) -> u128 {
    let lhs = a as u128;
    let rhs = b as u128;
    lhs * rhs
}

/// Barrett reduction of 128-bit integer modulo 64-bit integer. Source: Menezes,
/// Alfred; Oorschot, Paul; Vanstone, Scott. Handbook of Applied Cryptography,
/// Section 14.3.3.
/// @param a: operand (128-bit)
/// @param m: modulus (64-bit)
/// @param mu: 2^128/modulus (128-bit)
/// @return result: 64-bit result = a mod m
#[inline]
pub fn barrett_reduction(a: u128, modulus: u64, mu: u128) -> u64 {
    // (a * mu)/2^128 // need the upper 128-bit of (256-bit product)

    let a_lo = a as u64;
    let a_hi = (a >> 64) as u64;
    let mu_lo = mu as u64;
    let mu_hi = (mu >> 64) as u64;

    let left_hi = (mul_128(a_lo, mu_lo) >> 64) as u64; // mul left parts, discard lower word

    let middle = mul_128(a_lo, mu_hi); // mul middle first
    let middle_lo = middle as u64;
    let middle_hi = (middle >> 64) as u64;

    // accumulate and check carry
    let (tmp1, carry) = middle_lo.overflowing_add(left_hi);
    let carry = if carry { 1 } else { 0 };

    let tmp2 = middle_hi + carry; // accumulate

    let middle = mul_128(a_hi, mu_lo); // mul middle second
    let middle_lo = middle as u64;
    let middle_hi = (middle >> 64) as u64;

    let (_, carry) = middle_lo.overflowing_add(tmp1); // check carry
    let carry = if carry { 1 } else { 0 };

    let left_hi = middle_hi + carry; // accumulate

    // now we have the lower word of (a * mu)/2^128, no need for higher word
    let tmp1 = a_hi * mu_hi + tmp2 + left_hi;

    // subtract lower words only, higher words should be the same
    let mut result = a_lo - tmp1 * modulus;

    while result >= modulus {
        result -= modulus;
    }

    result
}
