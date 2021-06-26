use crate::Msg;
use seed::prelude::*;

pub fn render() -> Node<Msg> {
    div![
        h1!["Archives"],
        br![],
        br![],
        br![],
        span![
            a![
                "Mersenne",
                attrs! {At::Class => "link", At::Href => "/archive/mersenne/"}
            ],
            br![],
            a![
                "Perfect",
                attrs! {At::Class => "link", At::Href => "/archive/perfect/"}
            ],
            br![],
            a![
                "Prime",
                attrs! {At::Class => "link", At::Href => "/archive/prime/"}
            ],
        ],
    ]
}
