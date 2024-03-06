use crate::Msg;
use seed::prelude::*;

/// # Generate html using seed macros
pub fn render() -> Node<Msg> {
    div![
        h1!["404"],
        br![],
        br![],
        br![],
        b!["If you were expecting something to be here, please file a bug/issue on "],
        a![
            "github",
            attrs! {At::Class => "link", At::Href => "https://github.com/craigmayhew/bigprimes.net/issues/"}
        ],
        "."
    ]
}
