use crate::Msg;
use seed::prelude::*;

extern crate num_bigint as bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

use crate::utils::nth;

const NUMBERS_PER_PAGE: usize = 25;

/// Return a vector of fibonnaci numbers from n to n+count
fn nth_fibonacci(n: usize, count: usize) -> Vec<BigUint> {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    let mut x = Vec::new();
    for i in 0..(n + count - 1) {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
        if i >= n - 1 {
            x.push(f0.to_owned());
        }
    }
    x
}

/// Add a space character every 3 ascii characters, starting from the right of the String
fn add_space_every_3_ascii_chars_from_right(s: &str) -> String {
    // Convert the input string to bytes
    let bytes = s.as_bytes();

    // Create a vector with preallocated capacity to store the result.
    // The capacity is the original length plus space for potential spaces.
    let mut result = Vec::with_capacity(bytes.len() + (bytes.len() / 3));

    let mut count = 0;
    // Iterate over the bytes in reverse order to start adding spaces from the right
    for byte in bytes.iter().rev() {
        if count == 3 {
            // Insert a space after every 3 characters
            result.push(b' ');
            count = 0;
        }
        // Push the current byte to the result
        result.push(*byte);
        count += 1;
    }

    // Reverse the result to restore the original order
    result.reverse();

    // Convert the byte vector back to a String.
    // Since we started with valid UTF-8 single-byte characters, this will be safe.
    String::from_utf8(result).expect("Error when converting bytes to string")
}

/// # Generate html using seed macros
pub fn render(slug: String) -> Node<Msg> {
    let slug_int: usize = slug.parse().unwrap();
    let fib_vec = nth_fibonacci(slug_int, NUMBERS_PER_PAGE);

    let mut fib_vec_formatted = Vec::with_capacity(NUMBERS_PER_PAGE);
    for i in 0..NUMBERS_PER_PAGE {
        fib_vec_formatted.push(p![add_space_every_3_ascii_chars_from_right(
            &fib_vec[i].to_string()
        )]);
    }

    let prev_link: Vec<Node<_>> = if slug_int <= 1 {
        Vec::new()
    } else if slug_int <= NUMBERS_PER_PAGE {
        vec![a![
            "back to 1st fibonacci numbers",
            attrs! {At::Class => "link", At::Href => "/archive/fibonacci/1/".to_string()}
        ]]
    } else {
        vec![a![
            "previous ",
            NUMBERS_PER_PAGE.to_string(),
            " fibonacci numbers",
            attrs! {At::Class => "link", At::Href => format!(
                "/archive/fibonacci/{}/",
                &(slug_int - NUMBERS_PER_PAGE).to_string()
            )}
        ]]
    };

    div![
        h1!["The Fibonacci Numbers"],
        br![],
        "This page shows the ",
        nth(slug_int),
        " fibonacci number followed by the next ",
        (NUMBERS_PER_PAGE - 1).to_string(),
        ".",
        br![],
        br![],
        fib_vec_formatted,
        br![],
        prev_link,
        br![],
        a![
            "next ",
            NUMBERS_PER_PAGE.to_string(),
            " fibonacci numbers",
            attrs! {At::Class => "link", At::Href => format!(
                "/archive/fibonacci/{}/",
                &(slug_int + NUMBERS_PER_PAGE).to_string()
            )}
        ],
        br![],
        br![],
        a![
            "100th Fibonacci Number",
            attrs! {At::Class => "link", At::Href => "/archive/fibonacci/100/"}
        ],
        br![],
        a![
            "1000th Fibonacci Number",
            attrs! {At::Class => "link", At::Href => "/archive/fibonacci/1000/"}
        ],
        br![],
        a![
            "10000th Fibonacci Number",
            attrs! {At::Class => "link", At::Href => "/archive/fibonacci/10000/"}
        ],
    ]
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn nth_fibonacci_test() {
        let f0: BigUint = Zero::zero();
        let f1: BigUint = One::one();
        let f2: BigUint = &f0 + &f1;
        let f3: BigUint = &f1 + &f2;
        let f4: BigUint = &f2 + &f3;
        let f5: BigUint = &f3 + &f4;
        let f6: BigUint = &f4 + &f5;
        let f7: BigUint = &f5 + &f6;
        let f8: BigUint = &f6 + &f7;
        let f9: BigUint = &f7 + &f8;
        let f10: BigUint = &f8 + &f9;
        let f11: BigUint = &f9 + &f10;

        assert_eq!(nth_fibonacci(1, 2), vec![f1, f2]);
        assert_eq!(nth_fibonacci(3, 6), vec![f3, f4, f5, f6, f7, f8]);
        assert_eq!(nth_fibonacci(9, 3), vec![f9, f10, f11]);
    }

    #[bench]
    fn nth_fibonacci_bench(b: &mut Bencher) {
        b.iter(|| nth_fibonacci(12300, 9));
    }

    #[test]
    fn add_space_every_3_ascii_chars_from_right_test() {
        assert_eq!(add_space_every_3_ascii_chars_from_right("34"), "34");
        assert_eq!(add_space_every_3_ascii_chars_from_right("123"), "123");
        assert_eq!(add_space_every_3_ascii_chars_from_right("1234"), "1 234");
    }

    #[bench]
    fn add_space_every_3_ascii_chars_from_right_bench(b: &mut Bencher) {
        b.iter(|| {
            add_space_every_3_ascii_chars_from_right(
                "123456789123456789123456789123456789123456789123456789",
            )
        });
    }
}
