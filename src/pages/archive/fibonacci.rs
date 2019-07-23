use seed::prelude::*;
use crate::Msg;

fn nth(num: isize) -> String {
    format!("{}{}", num, match (num % 10, num % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    })
}

pub fn render(slug:String) -> seed::dom_types::Node<Msg> {
    let slug_int:isize = slug.parse().unwrap();

    div![
        h1!["The Fibonacci Numbers"],
        br![],
        "This page shows the ",nth(slug_int)," fibonacci number followed by the next 0.",
        br![],
        br![],
        a!["previous 25 fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],
        a!["next 25 fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],

        
        a!["100th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/100/"}],
        a!["1000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/1000/"}],
        a!["10000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/10000/"}],
    ]
}