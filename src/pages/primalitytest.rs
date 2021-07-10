use crate::Msg;
use seed::prelude::*;

fn trial_divide(n: usize, max: usize) -> usize {
    // Trial divides the positive integer n by the primes from 2 to max
    // Returns the first prime divisor found, or 0 if none found
    // Note: if n < max^2 is a prime, then n will be returned.
    if n % 2 == 0 {
        2
    } else if n % 3 == 0 {
        3
    } else {
        // No need to go past the square root of our number
        let sqrt: usize = (n as f64).sqrt().floor() as usize;
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

pub fn render() -> Node<Msg> {
    div![
        h1!["Primality Test, Calculate Primes"],
        br![],
        br![],
        br![],
        //TODO: Convert the j.js file into rust!
        Script!(attrs! {At::Src => "https://static.bigprimes.net/j.js"}),
        table![
            attrs! {At::Class => "text", At::Width => "300", At::Style => "border:1px solid #444; background-color:#e0faed"},
            tr![td![
                attrs! {At::Style => "padding: 10px"},
                form![
                    attrs! {At::Name => "primetest", At::OnSubmit => "return false"},
                    "Tool is limited to checking numbers upto 16 digits.",
                    br![],
                    br![],
                    "Is ",
                    input![
                        attrs! {At::Type => "text", At::Size => "16", At::Name => "input", At::Value => "", At::MaxLength => "16"}
                    ],
                    " prime? ",
                    button![attrs! {At::OnClick => "check(false,0)"}, "Check!"],
                    br![],
                    br![],
                    textarea![
                        attrs! {At::Name => "javascriptoutput", At::Cols => 60, At::Rows => 2, At::Disabled => "disabled"}
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
                form![
                    attrs! {At::Name => "primelist", At::OnSubmit => "return false"},
                    "This tool is limited to checking numbers upto 15 digits.",
                    br![],
                    br![],
                    "This will show ",
                    input![
                        attrs! {At::Type => "text", At::Size => "4", At::Name => "primes", At::Value => "1", At::MaxLength => "2"}
                    ],
                    " prime numbers after ",
                    input![
                        attrs! {At::Type => "text", At::Size => "16", At::Name => "start", At::Value => "0", At::MaxLength => "15"}
                    ],
                    " ",
                    button![
                        attrs! {At::OnClick => "primelist.javascriptlistoutput.value='';listy();"},
                        "Go!"
                    ],
                    br![],
                    br![],
                    textarea![
                        attrs! {At::Name => "javascriptlistoutput", At::Cols => 60, At::Rows => 10, At::Disabled => "disabled"}
                    ]
                ]
            ]]
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
