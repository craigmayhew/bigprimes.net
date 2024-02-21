use crate::Msg;
use seed::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone)]
pub struct PrimalityTestPageInputs {
    pub number: u64,
    pub primes: u64,
    pub start: u64,
}

fn trial_divide(n: u64, max: u64) -> u64 {
    // Trial divides the positive integer n by the primes from 2 to max
    // Returns the first prime divisor found, or 0 if none found
    // Note: if n < max^2 is a prime, then n will be returned.
    if n % 2 == 0 {
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

// modadd(a,b,N) finds a+b (mod N) where a, b, and N can be
// up to (2^53-1)/2.  Might up this to 2^53-1 eventually...
fn modadd(a: u64, b: u64, n: u64) -> u64 {
    // When the integers a, b satisfy a+b > 2^53-1, then (a+b)%N is wrong
    // so we add this routine to allow us to reach a, b = 2^53-1.
    if a + b > 9007199254740991 {
        // Could reduce a and b (mod N) here, but assuming that has already been done
        // won't hurt if not... subtract 2^52 from one, 2^52-1 from the other and the
        // add it back modulo N (MaxInt+1)
        let t = ((a - 4503599627370496) + (b - 4503599627370495)) % n;
        return t + (9007199254740991 % n);
    }
    // Usual case: a + b is not too large:
    (a + b) % n
}

fn modmult(mut a: u64, mut b: u64, n: u64) -> u64 {
    if a > n {
        a = a % n;
    }
    if b > n {
        b = b % n;
    }
    if a * b <= 9007199254740991 {
        return (a * b) % n;
    } else {
        if b > a {
            return modmult(b.clone(), a, n);
        }

        // Right to left binary multiplication
        let mut t = 0;
        let mut f = a;
        while b > 1 {
            if (b & 1) == 1 {
                t = modadd(t, f, n);
            }
            b >>= 1;
            f = modadd(f, f, n);
        }
        t = modadd(t, f, n);
        return t;
    }
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

// sprp(N,a) checks if N (odd!) is a strong probable prime base a
// (returns true or false)
fn sprp(n: u64, a: u64) -> bool {
    let mut d = n - 1;
    let mut s = 1; // Assumes n is odd!
    loop {
        if d % 2 == 1 {
            break;
        }
        d /= 2;
        s += 1;
    }
    // Now n-1 = d*2^s with d odd
    let mut b = modpow(a, d, n);
    if b == 1 {
        return true;
    }
    if b + 1 == n {
        return true;
    }
    while s > 1 {
        b = modmult(b, b, n);
        if b + 1 == n {
            return true;
        }
        s = s - 1;
    }
    false
}

pub struct Check {
    pub is_prime: bool,
    pub result: String,
}

pub fn check(input: String) -> Check {
    let trial_limit = 1300; // Should be bigger, like 10000
    let n: u64 = input.parse::<u64>().unwrap();
    let result;
    let mut is_prime = false;

    if n > 9007199254740991 {
        result = "Sorry, this routine will only handle integers below 9007199254740991.".to_owned();
    } else if n == 1 {
        result = "The number 1 is neither prime or composite (it is the multiplicative identity)."
            .to_owned();
    } else if n < 1 {
        result =
            "We usually restrict the terms prime and composite to positive integers".to_owned();
    } else {
        // Okay, n is of a resonable size, lets trial divide
        let i = trial_divide(n, trial_limit);
        if i > 0 && i != n {
            result = format!("{} is not a prime! It is {} * {}", n, i, n / i);
        } else if n < trial_limit * trial_limit {
            result = format!("{} is a (proven) prime!", n);
            is_prime = true;
        } else if sprp(n, 2)
            && sprp(n, 3)
            && sprp(n, 5)
            && sprp(n, 7)
            && sprp(n, 11)
            && sprp(n, 13)
            && sprp(n, 17)
        {
            // Some of these tests are unnecessary for small numbers, but for
            // small numbers they are quick anyway.
            if n < 341550071728321 {
                result = format!("{} is a (proven) prime.", n);
                is_prime = true;
            } else if n == 341550071728321 {
                result = format!("{} is not a prime! It is 10670053 * 32010157.", n);
            } else {
                result = format!(
                    "{} is probably a prime (it is a sprp bases 2, 3, 5, 7, 11, 13 and  17).",
                    n
                );
                is_prime = true;
            };
        } else {
            result = format!(
                "{} is (proven) composite (failed sprp test base {}).",
                n, 17
            );
        };
    };

    Check { is_prime, result }
}

pub fn listy(start_number: u64, number_of_primes: u64) -> String {
    let mut i = 0;
    let mut j = start_number;
    let mut list: String = "".to_string();
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
    el_output_textarea.set_inner_html(&listy(
        el_start_value.parse::<u64>().unwrap(),
        el_primes_value.parse::<u64>().unwrap(),
    ));
    ()
}

pub fn render(model: &crate::Model) -> Node<Msg> {
    div![
        h1!["Primality Test, Calculate Primes"],
        br![],
        br![],
        br![],
        table![
            attrs! {At::Class => "text", At::Width => "300", At::Style => "border:1px solid #444; background-color:#e0faed"},
            tr![td![
                attrs! {At::Style => "padding: 10px"},
                div![
                    attrs! {At::Id => "primetest"},
                    "Tool is limited to checking numbers upto 16 digits.",
                    br![],
                    br![],
                    "Is ",
                    input![
                        attrs! {At::Type => "number", At::Size => "19", At::Id => "number", At::Value => model.primalitycheckerfieldvalues.number.to_string(), At::MaxLength => "16"},
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
            attrs! {At::Class => "text", At::Width => "300", At::Style => "border:1px solid #444; background-color:#e0faed"},
            tr![td![
                attrs! {At::Style => "padding: 10px"},
                div![
                    attrs! {At::Id => "primelist"},
                    "This tool is limited to checking numbers upto 15 digits.",
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

    #[bench]
    fn mod_mult_bench(b: &mut Bencher) {
        b.iter(|| modmult(3, 3, 4));
        b.iter(|| modmult(110, 4, 7));
    }
}
