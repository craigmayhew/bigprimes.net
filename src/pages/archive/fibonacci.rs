use crate::Msg;
use seed::prelude::*;

extern crate num_bigint as bigint;
extern crate num_traits;
extern crate test;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

use crate::utils::nth;

/*
 * Return a vector of fibonnaci numbers from n to n+count
 */
fn nth_fibonacci(n: usize, count: usize) -> Vec<BigUint> {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    let mut x = vec![];
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

fn add_space_every_3_chars_from_right(s: &str) -> String {
    // First, reverse the input string to start adding spaces from the end
    let reversed: String = s.chars().rev().collect();

    // Process the reversed string in chunks of 3 characters
    let chunks: Vec<String> = reversed
        .as_bytes()
        // Split the reversed byte slice into chunks of 3
        .chunks(3)
        // Map each chunk: convert back to &str, reverse characters, and collect into a String
        .map(|chunk| std::str::from_utf8(chunk).unwrap().chars().rev().collect())
        // Collect the processed chunks into a Vec<String>
        .collect();

    // Join the chunks with spaces and reverse the resulting string to restore original order
    chunks.join(" ").chars().rev().collect()
}

pub fn render(slug: String) -> Node<Msg> {
    let numbers_per_page: usize = 25;
    let slug_int: usize = slug.parse().unwrap();
    let fib_vec = nth_fibonacci(slug_int, numbers_per_page);

    let mut fib_vec_formatted = Vec::with_capacity(numbers_per_page);
    for i in 0..numbers_per_page {
        fib_vec_formatted.push(p![add_space_every_3_chars_from_right(
            &fib_vec[i].to_string()
        )]);
    }

    let href_prev: String;
    let prev_link: Vec<Node<_>>;
    if slug_int <= 1 {
        prev_link = vec![];
    } else if slug_int <= 25 {
        href_prev = "/archive/fibonacci/1/".to_string();
        prev_link = vec![a![
            "back to 1st fibonacci numbers",
            attrs! {At::Class => "link", At::Href => href_prev}
        ]];
    } else {
        href_prev = format!(
            "/archive/fibonacci/{}/",
            &(slug_int - numbers_per_page).to_string()
        );
        prev_link = vec![a![
            "previous ",
            numbers_per_page.to_string(),
            " fibonacci numbers",
            attrs! {At::Class => "link", At::Href => href_prev}
        ]];
    }
    let href_next: String = format!(
        "/archive/fibonacci/{}/",
        &(slug_int + numbers_per_page).to_string()
    );

    div![
        h1!["The Fibonacci Numbers"],
        br![],
        "This page shows the ",
        nth(slug_int),
        " fibonacci number followed by the next ",
        (numbers_per_page - 1).to_string(),
        ".",
        br![],
        br![],
        fib_vec_formatted,
        br![],
        prev_link,
        br![],
        a![
            "next ",
            numbers_per_page.to_string(),
            " fibonacci numbers",
            attrs! {At::Class => "link", At::Href => href_next}
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
    fn nth_fibonaccit_bench(b: &mut Bencher) {
        b.iter(|| nth_fibonacci(12300, 9));
    }
}
