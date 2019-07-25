use seed::prelude::*;
use crate::Msg;

extern crate num_bigint as bigint;
extern crate num_traits;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

use crate::utils::nth;

pub fn render(slug:String) -> seed::dom_types::Node<Msg> {
    div![
        h1!["Number Cruncher"],
        "Welcome to the number cruncher.
        Type in a number in the box below and we will crunch it for you.<br><br>
        You will be taken to a page that tells you the following information about your number:",
        ul![
            li!["Is it odd or even?"],
            li!["Is it a palindrome?"],
            li!["Is it a prime number? (Checks numbers upto 11 digits in length)"],
            li!["Is it a ",a!["mersenne prime", attrs!{At::Class => "link", At::Href => "https://en.wikipedia.org/wiki/Mersenne_prime"}],"? (Checks numbers upto 2993 digits in length)"],
            li!["Is it a ",a!["fermat prime", attrs!{At::Class => "link", At::Href => "https://www.fermatsearch.org/"}],"? (Checks numbers upto 155 digits in length"],
            li!["Is it a ",a!["perfect number", attrs!{At::Class => "link", At::Href => "https://en.wikipedia.org/wiki/Perfect_number"}],"? (Checks numbers upto 12003 digits in length)"],
            li!["Is it a triangle number? (Checks numbers upto 40 digits in length)"],
            li!["Is it a square number?"],
            li!["Is it a cube number? (Checks numbers upto 17 digits in length)"],
            li!["Is it a factorial number?"],
            br![],
            br![],
            li!["All factors of the number will be listed (for numbers upto 9 digits)"],
            li!["The page will also show a list of base conversions. e.g. binary, octal and hexadecimal (Numbers upto 500 digits in length)"],
            li!["The number will be converted to roman numerals (Upto 6 digits in length)"],
            li!["The number will be converted to egyptian numerals (Upto 7 digits in length)"],
            li!["The number will be converted to chinese numerals (Upto 6 digits in length)"],
            li!["The number will be converted to babylonian numerals (Upto 13 digits in length)"],
        ],
        br![],
        br![],
	    "Please type your number here:",
        form![attrs!{At::Name => "crunchy", At::Action => "/cruncher/", At::Method => "get", At::Target => "_top"},
            div![
                textarea![attrs!{At::Name => "number", At::Cols => "85", At::Rows => "10", At::OnKeyDown => "if (event.keyCode == 13){document.location='/cruncher/'+crunchy.number.value+'/'}"}],
                br![],
                input![attrs!{At::Type => "button", At::Value => "Crunch", At::OnClick => "document.location='/cruncher/'+crunchy.number.value+'/'"}],
            ],
        ],
    ]
}
