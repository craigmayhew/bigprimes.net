use crate::Msg;
use seed::prelude::*;

/// # Generate html using seed macros
pub fn render() -> Node<Msg> {
    div![
        h1!["Contact"],
        br![],
        br![],
        br![],
        span![
            "Please contact me via ",
            a![
                "twitter",
                attrs! {At::Class => "link", At::Href => "https://twitter.com/craigmayhew"}
            ],
            " or by opening an issue on ",
            a![
                "github",
                attrs! {At::Class => "link", At::Href => "https://github.com/craigmayhew/bigprimes.net/"}
            ],
            ".",
        ],
    ]
}
