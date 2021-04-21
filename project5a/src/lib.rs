pub mod mandelbrot;
pub mod points;

/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    unimplemented!()
}

/**
 * Adds one to the referenced integer.
 */
pub fn add_one(n: &mut i32) {
    unimplemented!()
}

/**
 * Modifies the input string so that every character is replaced with 'a'.
 * See https://doc.rust-lang.org/std/string/struct.String.html.
 */
pub fn rewrite_string(s: &mut String) {
    unimplemented!()
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array.
    Index 0 stores the high-order bit.

    Conversion is unsigned (all place values are positive).
    
    Ex: to_decimal of [true, false, true, false] returns 10
**/
pub fn to_decimal(ls: [bool; 32]) -> u32 {
    unimplemented!()
}

/**
 * The Collatz conjecture (https://en.wikipedia.org/wiki/Collatz_conjecture) is that
 * when one repeatedly applies a function 'f' to a positive integer,
 * the output will eventually be 1. 'f' is defined as follows:
 * 
 * f(n) = n/2 if n is even
 * f(n) = 3n + 1 if n is odd
 * 
 * The function collatz(n) returns the number of times one must apply f to n to get 1.
 * For example, f(4) = 2, f(2) = 1. That is, it took 2 applications of f to get from 4 to 1.
 * Therefore, collatz(4) = 2.
*/
pub fn collatz(mut n: u64) -> u64 {
    unimplemented!()
}

/**
 * Returns a vector containing collatz(1), collatz(2), ..., collatz(n).
 */
pub fn collatz_times(n: u64) -> Vec<u64> {
    unimplemented!()
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    unimplemented!()
}

/**
 * Returns a new string whose contents is the concatenation of the two input slices.
 */
pub fn concatenate_strings(s1: &str, s2: &str) -> String {
    unimplemented!()
}
