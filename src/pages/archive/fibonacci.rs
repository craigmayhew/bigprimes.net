use seed::prelude::*;
use crate::Msg;

pub fn render() -> seed::dom_types::Node<Msg> {
    
    div![
        h1!["The Fibonacci Numbers"],
        br![],
        "This page shows the 70333rd fibonacci number followed by the next 0.",
        br![],
        br![],
        a!["previous 25 fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],
        a!["next 25 fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],

        
        a!["100th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/100/"}],
        a!["1000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/1000/"}],
        a!["10000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/10000/"}],
    ]
}