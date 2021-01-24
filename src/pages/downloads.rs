use crate::Msg;
use seed::prelude::*;

pub fn render() -> seed::dom_types::Node<Msg> {
    //TODO: `svg` has type `seed::dom_types::Node<Msg>`, which does not implement the `Copy` trait, therefore we must have duplicate code for now.
    let svg_attrs = attrs! {
            At::Width => "20px";
            At::Height => "20px";
            At::ViewBox => "-128 500 1024 1";
    };

    let path_attrs = attrs! {
        At::Fill => "black";
        At::D => "M320 576v-64h-64v64H320zM320 448v-64h-64v64H320zM320 320v-64h-64v64H320zM192 384h64v-64h-64V384zM576 64H0v896h768V256L576 64zM704 896H64V128h192v64h64v-64h192l192 192V896zM192 256h64v-64h-64V256zM192 512h64v-64h-64V512zM192 640l-64 64v128h256V704l-64-64h-64v-64h-64V640zM320 704v64H192v-64H320z"
    };

    let svg1 = svg![&svg_attrs, path![&path_attrs],];

    let svg2 = svg![&svg_attrs, path![&path_attrs],];

    let svg3 = svg![&svg_attrs, path![&path_attrs],];

    div![
        h1!["Downloads"],
        br![],
        br![],
        br![],
        table![tbody![
            tr![
                td!["First 44 known Mersenne primes"],
                td![a![
                    svg1,
                    attrs! {At::Class => "link", At::Href => "https://static.bigprimes.net/archive/mersenne/Mersenne Primes.zip"}
                ],],
            ],
            tr![
                td!["First 12 factored Fermat numbers"],
                td![a![
                    svg2,
                    attrs! {At::Class => "link", At::Href => "https://static.bigprimes.net/archive/fermat/Fermat Numbers.zip"}
                ],],
            ],
            tr![
                td!["First 44 known Perfect numbers"],
                td![a![
                    svg3,
                    attrs! {At::Class => "link", At::Href => "https://static.bigprimes.net/archive/perfect/Perfect Numbers.zip"}
                ],],
            ]
        ]],
    ]
}
