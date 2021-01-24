use crate::Msg;
use seed::prelude::*;

pub fn render() -> seed::dom_types::Node<Msg> {
    div![
        h1!["Status"],
        br![],
        br![],
        br![],
        table![tbody![
            tr![td!["Number of verified primes:"], td!["1.4 billion"],],
            tr![
                td!["Website build status:"],
                td![img![
                    attrs! {At::Src => "https://travis-ci.org/craigmayhew/bigprimes.net.svg?branch=master"}
                ],],
            ]
        ]],
    ]
}
