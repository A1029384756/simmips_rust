use std::mem::size_of_val;

pub fn sign_extend(v: i32, n_bits: u32) -> i32 {
    let other_bits = size_of_val(&v) as u32 * 8 - n_bits;
    v.wrapping_shl(other_bits).wrapping_shr(other_bits)
}
