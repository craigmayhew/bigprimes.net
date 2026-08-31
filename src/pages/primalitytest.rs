use crate::Msg;
use seed::prelude::*;
use web_sys::HtmlInputElement;

const INVALID_NUMBER_MESSAGE: &str =
    "Please enter an unsigned 64-bit integer in decimal or 0x-prefixed hexadecimal notation.";
// These witnesses make Miller-Rabin deterministic for every value in the u64 range.
const DETERMINISTIC_MILLER_RABIN_BASES: [u64; 7] =
    [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];

#[derive(Clone)]
pub struct PrimalityTestPageInputs {
    pub number: String,
    pub primes: String,
    pub start: String,
}

fn parse_number(input: &str) -> Result<u64, &'static str> {
    let input = input.trim();
    if input.is_empty() {
        return Err(INVALID_NUMBER_MESSAGE);
    }

    let parsed = if let Some(hex_digits) = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
    {
        u64::from_str_radix(hex_digits, 16)
    } else {
        input.parse::<u64>()
    };

    parsed.map_err(|_| INVALID_NUMBER_MESSAGE)
}

fn trial_divide(n: u64, max: u64) -> u64 {
    // Trial divides the positive integer n by the primes from 2 to max
    // Returns the first prime divisor found, or 0 if none found
    // Note: if n < max^2 is a prime, then n will be returned.
    if n & 1 == 0 {
        2
    } else if n % 3 == 0 {
        3
    } else {
        // No need to go past the square root of our number
        let sqrt: u64 = (n as f64).sqrt().floor() as u64;
        let stop = if sqrt > max { max } else { sqrt };
        // Okay, lets "wheel factor" alternately adding 2 and 4
        let mut di = 2;
        let mut i = 5;
        while i <= stop {
            if n % i == 0 {
                return i;
            }
            i += di;
            di = 6 - di;
        }
        if n >= max * max {
            0
        } else {
            n
        }
    }
}

fn modmult(a: u64, b: u64, n: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(n)) as u64
}

// modpow(a,exp,N) finds a^exp (mod N) where a, b, and N are
// limited by modmult
fn modpow(a: u64, mut exp: u64, n: u64) -> u64 {
    if exp == 0 {
        return 1;
    }

    // Right to left binary exponentiation
    let mut t = 1;
    let mut f = a;
    while exp > 1 {
        if (exp & 1) == 1 {
            // if exponent is odd
            t = modmult(t, f, n);
        }
        exp >>= 1;
        f = modmult(f, f, n);
    }
    modmult(t, f, n)
}

// Checks whether odd n is a strong probable prime to base a.
fn sprp(n: u64, a: u64) -> bool {
    let a = a % n;
    if a == 0 {
        return true;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }

    let mut b = modpow(a, d, n);
    if b == 1 || b == n - 1 {
        return true;
    }

    for _ in 1..s {
        b = modmult(b, b, n);
        if b == n - 1 {
            return true;
        }
    }

    false
}

pub struct Check {
    pub is_prime: bool,
    pub result: String,
}

pub fn check(input: String) -> Check {
    let n = match parse_number(&input) {
        Ok(n) => n,
        Err(message) => {
            return Check {
                is_prime: false,
                result: message.to_owned(),
            };
        }
    };

    let trial_limit = 1300; // Should be bigger, like 10000
    let result;
    let mut is_prime = false;

    if n == 1 {
        result = "The number 1 is neither prime nor composite (it is the multiplicative identity)."
            .to_owned();
    } else if n == 0 {
        result =
            "We usually restrict the terms prime and composite to positive integers".to_owned();
    } else {
        let i = trial_divide(n, trial_limit);
        if i > 0 && i != n {
            result = format!("{} is not a prime! It is {} * {}", n, i, n / i);
        } else if n < trial_limit * trial_limit {
            result = format!("{} is a (proven) prime!", n);
            is_prime = true;
        } else if let Some(base) = DETERMINISTIC_MILLER_RABIN_BASES
            .iter()
            .find(|&&base| !sprp(n, base))
        {
            result = format!(
                "{} is (proven) composite (failed sprp test base {}).",
                n, base
            );
        } else {
            result = format!("{} is a (proven) prime.", n);
            is_prime = true;
        }
    };

    Check { is_prime, result }
}

pub fn listy(start_number: u64, number_of_primes: u64) -> String {
    let mut i = 0;
    let mut j = start_number;
    let mut list: String = String::new();
    while i < number_of_primes {
        let result = check(j.to_string());
        if result.is_prime == true {
            list = list + &result.result + "\n";
            i += 1;
        }
        j += 1;
    }
    list
}

pub fn go_crunch() -> () {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let el_input_value = document
        .get_element_by_id("number")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    let el_output_textarea = document
        .get_element_by_id("prime_check_output")
        .expect("missing output textarea");
    el_output_textarea.set_inner_html(&check(el_input_value).result);
    ()
}

pub fn go_list() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let el_start_value = document
        .get_element_by_id("start")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    let el_primes_value = document
        .get_element_by_id("primes")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    let el_output_textarea = document
        .get_element_by_id("prime_check_list_output")
        .expect("missing output textarea");
    let output = match (
        parse_number(&el_start_value),
        parse_number(&el_primes_value),
    ) {
        (Ok(start), Ok(primes)) => listy(start, primes),
        _ => "Please enter valid unsigned integers for the start and count.".to_owned(),
    };
    el_output_textarea.set_inner_html(&output);
    ()
}

pub fn render(model: &crate::Model) -> Node<Msg> {
    div![
        h1!["Primality Test, Calculate Primes"],
        br![],
        br![],
        br![],
        table![
            attrs! {At::Class => "primality-tool text", At::Width => "300"},
            tr![td![
                attrs! {At::Class => "primality-tool-cell"},
                div![
                    attrs! {At::Id => "primetest"},
                    "Enter a decimal integer or prefix a hexadecimal value with 0x. The full unsigned 64-bit range is supported.",
                    br![],
                    br![],
                    "Is ",
                    input![
                        attrs! {At::Type => "text", At::Size => "22", At::Id => "number", At::Value => model.primalitycheckerfieldvalues.number.to_string(), At::MaxLength => "20"},
                        input_ev(Ev::Input, |val| {
                            Msg::PrimalityCheckerInputNumberValueChanged(val)
                        }),
                    ],
                    " prime? ",
                    button![
                        "Check!",
                        ev(Ev::Click, |_e| { Msg::PrimalityChecker(go_crunch()) })
                    ],
                    br![],
                    br![],
                    textarea![
                        attrs! {At::Id => "prime_check_output", At::Cols => 60, At::Rows => 2, At::Disabled => "disabled"}
                    ]
                ]
            ]]
        ],
        br![],
        br![],
        table![
            attrs! {At::Class => "primality-tool text", At::Width => "300"},
            tr![td![
                attrs! {At::Class => "primality-tool-cell"},
                div![
                    attrs! {At::Id => "primelist"},
                    "This tool is limited to checking numbers up to 15 digits.",
                    br![],
                    br![],
                    "This will show ",
                    input![
                        attrs! {At::Type => "number", At::Size => "4", At::Id => "primes", At::Value => model.primalitycheckerfieldvalues.primes.to_string(), At::MaxLength => "2"},
                        input_ev(Ev::Input, |val| {
                            Msg::PrimalityCheckerInputPrimesValueChanged(val)
                        }),
                    ],
                    " prime numbers after ",
                    input![
                        attrs! {At::Type => "number", At::Size => "19", At::Id => "start", At::Value => model.primalitycheckerfieldvalues.start.to_string(), At::MaxLength => "15"},
                        input_ev(
                            Ev::Input,
                            |val| Msg::PrimalityCheckerInputStartValueChanged(val)
                        ),
                    ],
                    " ",
                    button![
                        "Go!",
                        ev(Ev::Click, |_e| { Msg::PrimalityChecker(go_list()) })
                    ],
                    br![],
                    br![],
                    textarea![
                        attrs! {At::Id => "prime_check_list_output", At::Cols => 60, At::Rows => 10, At::Disabled => "disabled"}
                    ]
                ]
            ]]
        ]
    ]
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn parse_number_supports_decimal_and_hexadecimal() {
        assert_eq!(parse_number("31"), Ok(31));
        assert_eq!(parse_number("0x1f"), Ok(31));
        assert_eq!(parse_number("0X1F"), Ok(31));
        assert_eq!(parse_number("18446744073709551615"), Ok(u64::MAX));
        assert_eq!(parse_number("0xFFFFFFFFFFFFFFFF"), Ok(u64::MAX));
    }

    #[test]
    fn parse_number_rejects_invalid_or_out_of_range_input() {
        assert_eq!(parse_number(""), Err(INVALID_NUMBER_MESSAGE));
        assert_eq!(parse_number("0x"), Err(INVALID_NUMBER_MESSAGE));
        assert_eq!(parse_number("-1"), Err(INVALID_NUMBER_MESSAGE));
        assert_eq!(
            parse_number("18446744073709551616"),
            Err(INVALID_NUMBER_MESSAGE)
        );
    }

    #[test]
    fn check_accepts_issue_24_hexadecimal_input() {
        let result = check("0xFFFFFFFFFFFFF1".to_owned());

        assert!(!result.is_prime);
        assert_eq!(
            result.result,
            "72057594037927921 is not a prime! It is 59 * 1221315153185219"
        );
    }

    #[test]
    fn check_supports_primes_across_the_u64_range() {
        let result = check("18446744073709551557".to_owned());

        assert!(result.is_prime);
        assert_eq!(result.result, "18446744073709551557 is a (proven) prime.");
    }

    #[test]
    fn check_returns_an_error_for_invalid_input() {
        let result = check("not a number".to_owned());

        assert!(!result.is_prime);
        assert_eq!(result.result, INVALID_NUMBER_MESSAGE);
    }

    #[test]
    fn trial_divide_test() {
        assert_eq!(trial_divide(2, 2), 2);
        assert_eq!(trial_divide(3, 4), 3);
        assert_eq!(trial_divide(30, 10000), 2);
        assert_eq!(trial_divide(7777771, 10000), 29);
        assert_eq!(trial_divide(7777771111111111, 10000), 11);
        //in the case of a prime
        assert_eq!(trial_divide(777777111111113, 10000), 0);
    }

    #[bench]
    fn trial_divide_bench(b: &mut Bencher) {
        b.iter(|| trial_divide(7777771, 10000));
    }

    #[test]
    fn sprp_test() {
        assert_eq!(sprp(27, 5), false);
        assert_eq!(sprp(31, 7), true);
        assert_eq!(sprp(217, 7), false);
        assert_eq!(sprp(19, 13), true);
    }

    #[bench]
    fn sprp_bench(b: &mut Bencher) {
        b.iter(|| sprp(27, 5));
        b.iter(|| sprp(31, 7));
        b.iter(|| sprp(217, 7));
        b.iter(|| sprp(19, 13));
    }

    #[test]
    fn mod_pow_test() {
        assert_eq!(modpow(6, 3, 11), 7);
        assert_eq!(modpow(113, 7, 11), 9);
    }

    #[bench]
    fn mod_pow_bench(b: &mut Bencher) {
        b.iter(|| modpow(6, 3, 11));
        b.iter(|| modpow(113, 7, 11));
    }

    #[test]
    fn mod_mult_test() {
        assert_eq!(modmult(3, 3, 4), 1);
        assert_eq!(modmult(110, 4, 7), 6);
    }

    #[test]
    fn mod_mult_handles_u64_product_overflow() {
        let a: u64 = 9_007_199_254_740_000;
        let b: u64 = 9_007_199_254_739_000;
        let n: u64 = 9_007_199_254_740_991;

        assert!(a.checked_mul(b).is_none());
        assert_eq!(modmult(a, b, n), 1_973_081);
    }

    #[bench]
    fn mod_mult_bench(b: &mut Bencher) {
        b.iter(|| modmult(3, 3, 4));
        b.iter(|| modmult(110, 4, 7));
    }
}
