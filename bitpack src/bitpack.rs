/// function that transforms a signed i64 to an unsigned u64, returns a u64
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn signed_to_unsigned(n: i64, width: u64) -> u64 {
    // if the number is positive, it does not need to be changed
    if n >= 0 {
        return n as u64;
    }
    // if the number is negative, convert it as a two's complement u64
    else {
        (n + (1 << width)) as u64
    }
}

/// function that transforms an unsigned u64 to a signed i64, returns an i64
/// 
/// # Arguments:
/// * `n`: An unsigned integer value
/// * `width`: the width of a bit field
pub fn unsigned_to_signed(n: u64, width: u64) -> i64 {
    // if the number is less than half the width range, it does not need to be changed
    if n < (1 << (width - 1)) {
        return n as i64;
    }
    // if the number should be negative, convert it as a two's complement i64
    else {
        (n as i64) - (1 << width)
    }
}

/// Passes function to fitsu after changing signed i64s to unsigned u64s
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    // pass to fitsu as unsigned ints
    fitsu(signed_to_unsigned(n, width), width)
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if width >= 64 {
        // if the width is 64 or more, any u64 value can fit, return true
        true
    } 
    else {
        // Calculate the maximum value that can fit into the width
        let max_value = (1u64 << width) - 1;
        // If n is less than or equal to the maximum value, return true
        n <= max_value
    }
}

/// Passes function to getu after changing unsigned u64s to signed i64s
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    // pass to getu as unsigned ints
    unsigned_to_signed(getu(word, width, lsb), width)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {

    // mask to extract the desired bit field
    let mask = ((1u64 << width) - 1) << lsb;

    // return the extracted bit field
    (word & mask) >> lsb
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {

    // return None if n does not fit in width
    if !fitsu(value, width) {
        return None;
    }

    // mask to clear the bits in the specified bit field
    let clear_mask = !(((1u64 << width) - 1) << lsb);

    // mask to set the new value in the specified bit field
    let set_mask = value << lsb;

    // update the word and return the result
    let result = (word & clear_mask) | set_mask;
    Some(result)
}

/// Passes function to newu after changing signed i64s to unsigned u64s
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    // pass to newu as unsigned ints
    newu(word, width, lsb, signed_to_unsigned(value, width))
}