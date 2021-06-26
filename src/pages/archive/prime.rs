use crate::utils::nth;
use crate::Msg;
use regex::Regex;
use seed::prelude::*;

pub mod prime_utils {
    pub fn sieve(start: usize, end: usize) -> Vec<usize> {
        let mut is_prime = vec![true; end + 1];
        is_prime[0] = false;
        if end >= 1 {
            is_prime[1] = false
        }
        let sqrtlmt = (end as f64).sqrt() as usize + 1;

        for num in 2..sqrtlmt {
            if is_prime[num] {
                let mut num_squared = num * num;
                while num_squared <= end {
                    is_prime[num_squared] = false;
                    num_squared += num;
                }
            }
        }

        is_prime
            .iter()
            .enumerate()
            .filter_map(|(pr, &is_pr)| {
                if is_pr {
                    //is prime
                    if pr >= start {
                        //is greater than start vaariable
                        Some(pr) //prime and we want it returned
                    } else {
                        None // not prime
                    }
                } else {
                    None // not prime
                }
            })
            .collect()
    }

    pub fn sieve_n_primes(start: usize, end: usize, n_primes: usize) -> Vec<usize> {
        let sieved_numbers: Vec<usize> = sieve(start, end);
        // check if we didn't get as many numbers as we expected
        if n_primes >= sieved_numbers.len() {
            // todo: consider erroring in this case, as we asked for a
            // specific number of primes and did not provide them all
            sieved_numbers[0..sieved_numbers.len() - 1].to_vec()
        } else {
            sieved_numbers[0..n_primes].to_vec()
        }
    }
}

pub fn render(slug: String) -> Node<Msg> {
    let numbers_per_page: usize = 100;
    let col_count = 4;
    let numbers_per_col = numbers_per_page / col_count;

    // check that slug is a numeric string
    // and if not then
    let rgx = Regex::new(r"^([1-9]+[0-9]*)$").unwrap();
    let slug_is_int: bool = rgx.is_match(&slug);

    if !slug_is_int {
        div![
            h1!["The Prime Numbers"],
            br![],
            "Malformed page url, was the number correctly entered?"
        ]
    } else {
        let mut slug_int: usize = slug.parse().unwrap();
        if slug_int < 1 {
            slug_int = 1
        };

        // TODO: slug_int*100 and first_prime_on_page are both inefficient
        // first_prime_on_page could be multiplied by slightly more than 1, just enough to cover expected primes on the page
        // slug_int*100 would cover us up until prime gaps are above 100 on average
        let first_prime_on_page = prime_utils::sieve_n_primes(2, slug_int * 100, slug_int)
            .pop()
            .unwrap();

        let upper_bound_of_sieve;
        if slug_int < 1000 {
            upper_bound_of_sieve = 10000;
        } else {
            upper_bound_of_sieve = first_prime_on_page + 10000;
        }

        let primes = prime_utils::sieve_n_primes(
            first_prime_on_page,
            upper_bound_of_sieve,
            numbers_per_page,
        );

        let mut prime_vec_formatted = vec![];
        for col in 1..col_count + 1 {
            let mut prime_vec = vec![];
            for i in numbers_per_col * (col - 1)..numbers_per_col * col {
                // check data is available to display
                if i >= primes.len() {
                    continue;
                }

                // the data is there, display it
                let mut href: String = "/cruncher/".to_owned();
                href.push_str(&primes[i].to_string());
                prime_vec.push(a![
                    primes[i].to_string(),
                    attrs! {At::Class => "link", At::Href => href}
                ]);
                prime_vec.push(br![]);
            }

            prime_vec_formatted.push(div![prime_vec, attrs! {At::Class => "prime_archive_div"}]);
        }

        let href_prev: String;
        let prev_link: Vec<Node<_>>;
        // we are on the first page of primes so don't display a previous button
        if slug_int <= 1 {
            prev_link = vec![];
        //display a link with text "back to 1st prime numbers"
        } else if slug_int as isize - numbers_per_page as isize <= 0 {
            href_prev = "/archive/prime/1/".to_string();
            prev_link = vec![a![
                "back to 1st prime numbers",
                attrs! {At::Class => "link", At::Href => href_prev}
            ]];
        } else {
            href_prev = format!(
                "/archive/prime/{}/",
                &(slug_int - numbers_per_page).to_string()
            );
            prev_link = vec![a![
                "previous ",
                numbers_per_page.to_string(),
                " prime numbers",
                attrs! {At::Class => "link", At::Href => href_prev}
            ]];
        }
        let href_next: String = format!(
            "/archive/prime/{}/",
            &(slug_int + numbers_per_page).to_string()
        );

        div![
            h1!["The Prime Numbers"],
            br![],
            "This page shows the ",
            nth(slug_int),
            " prime number followed by the next ",
            (numbers_per_page - 1).to_string(),
            ".",
            br![],
            br![],
            prime_vec_formatted,
            br![],
            br![],
            prev_link,
            br![],
            a![
                "next ",
                numbers_per_page.to_string(),
                " prime numbers",
                attrs! {At::Class => "link", At::Href => href_next}
            ],
            br![],
            br![],
            a![
                "100th prime Number",
                attrs! {At::Class => "link", At::Href => "/archive/prime/100/"}
            ],
            br![],
            a![
                "1000th prime Number",
                attrs! {At::Class => "link", At::Href => "/archive/prime/1000/"}
            ],
            br![],
            a![
                "10000th prime Number",
                attrs! {At::Class => "link", At::Href => "/archive/prime/10000/"}
            ],
            br![],
            a![
                "100000th prime Number",
                attrs! {At::Class => "link", At::Href => "/archive/prime/100000/"}
            ],
            br![],
            a![
                "1000000th prime Number",
                attrs! {At::Class => "link", At::Href => "/archive/prime/1000000/"}
            ],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*TODO:
    1) Introduce sane upper limit to prime archive. e.g. 2 or 3 seconds and then give up on the page.
    */

    #[test]
    fn sieve_test() {
        //test starting from 1
        assert_eq!(prime_utils::sieve(1, 10), vec![2, 3, 5, 7]);

        //test starting from 3
        assert_eq!(prime_utils::sieve(3, 10), vec![3, 5, 7]);

        //test starting from 3, return 15 numbers
        assert_eq!(
            prime_utils::sieve_n_primes(3, 100, 15),
            vec![3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53]
        );

        //test mid range prime numbers
        let start_nth_prime = 1600;
        let first_prime_on_page = prime_utils::sieve_n_primes(2, 50000, start_nth_prime)
            .pop()
            .unwrap();
        let upper_bound_of_sieve = first_prime_on_page * 2;
        assert_eq!(
            prime_utils::sieve_n_primes(first_prime_on_page, upper_bound_of_sieve, 3),
            vec![13499, 13513, 13523]
        );

        //test more mid range prime numbers
        let start_nth_prime = 4800;
        let first_prime_on_page = prime_utils::sieve_n_primes(2, 50000, start_nth_prime)
            .pop()
            .unwrap();
        let upper_bound_of_sieve = first_prime_on_page * 2;
        assert_eq!(
            prime_utils::sieve_n_primes(first_prime_on_page, upper_bound_of_sieve, 3),
            vec![46447, 46451, 46457]
        );
    }
}
