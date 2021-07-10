use crate::Msg;
use seed::prelude::*;

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
