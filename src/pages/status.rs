use crate::Msg;
use seed::prelude::*;

pub fn render() -> Node<Msg> {
    div![
        h1!["Status"],
        br![],
        br![],
        br![],
        table![tbody![
            tr![td!["Number of verified primes:"], td!["1.4 billion"],],
        ]],
    ]
}
