use seed::prelude::*;
use crate::Msg;

extern crate num_bigint as bigint;
extern crate num_traits;

use crate::utils::nth;
use regex::Regex;

mod numerics_to_text {
    use num_traits::{Num,ToPrimitive,Zero};
    use num_bigint::{BigUint,ToBigUint};

    pub fn convert(str_num: String) -> String{
        let units:Vec<&str> = vec![
            "",
            " one",
            " two",
            " three",
            " four",
            " five",
            " six",
            " seven",
            " eight",
            " nine",
            " ten",
            " eleven",
            " twelve",
            " thirteen",
            " fourteen",
            " fifteen",
            " sixteen",
            " seventeen",
            " eighteen",
            " nineteen"
        ];
        let tens:Vec<&str> = vec![
            "",
            "",
            " twenty",
            " thirty",
            " forty",
            " fifty",
            " sixty",
            " seventy",
            " eighty",
            " ninety"
        ];
        let triplets:Vec<&str> = vec![
            "",
            " thousand",
            " million",
            " billion",
            " trillion",
            " quadrillion",
            " quintillion",
            " sextillion",
            " septillion",
            " octillion",
            " nonillion",
            " decillion",
            " undecillion",
            " duodecillion",
            " tredecillion",
            " quattuordecillion",
            " quindecillion",
            " sexdecillion",
            " septendecillion",
            " octodecillion",
            " novemdecillion",
            " vigintillion",
            " unvigintillion",
            " duovigintillion",
            " tresvigintillion",
            " quattuorvigintillion",
            " quinquavigintillion",
            " sesvigintillion",
            " septemvigintillion",
            " octovigintillion",
            " novemvigintillion",
            " trigintillion",
            " untrigintillion",
            " duotrigintillion",
            " trestrigintillion",
            " quattuortrigintillion",
            " quinquatrigintillion",
            " sestrigintillion",
            " septentrigintillion",
            " octotrigintillion",
            " noventrigintillion",
            " quadragintillion",
            " ERROR ",
        ];

        let num:BigUint = num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap();
        convert_num(num, units, tens, triplets)
    }

    fn convert_num(num:BigUint, units:Vec<&str>, tens:Vec<&str>, triplets:Vec<&str>) -> String {
        let mut string:String = "".to_owned();
        if num != Zero::zero() {
            string.push_str("zero");
        } else {
            string = convert_tri(num, Zero::zero(), units, tens, triplets);
        }
        string
    }

    fn convert_tri(num:BigUint, tri:usize, units:Vec<&str>, tens:Vec<&str>, triplets:Vec<&str>) -> String {
        // chunk the number, ...rxyy
        let ten:BigUint = 10.to_biguint().unwrap();
        let hundred:BigUint = 100.to_biguint().unwrap();
        let thousand:BigUint = 1000.to_biguint().unwrap();
        let r = &num / &thousand; // this in theory is rounding down to an int
        let x:usize = ((&num / &hundred) % &ten).to_usize().unwrap();
        let y:usize = (&num % &hundred).to_usize().unwrap();
        // init the output string
        let mut string:String = "".to_owned();
        // do hundreds
        if x > 0 {
            string.push_str(units[x]);
            string.push_str(" hundred");
        }
        // do units and tens
        if y < 20 {
            string.push_str(units[y]);
        } else {
            string.push_str(tens[(y / 10)]);
            string.push_str(units[(y / 10)]);
        }
        // add triplet modifier only if there
        // is some output to be modified...
        if string != "" {
            match triplets[tri] {
                _s => string.push_str(&triplets[tri].to_string()),
            }
        }
        // continue recursing?
        if r > Zero::zero() {
            let mut string2:String = convert_tri(r, tri + 1, units, tens, triplets);
            string2.push_str(&string);
            string2
        } else {
            string
        }
    }

}

fn html_form() -> seed::dom_types::Node<Msg> {
    div![
        h1!["Number Cruncher"],
        "Welcome to the number cruncher.
        Type in a number in the box below and we will crunch it for you.",
        br![],
        br![],
        "You will be taken to a page that tells you the following information about your number:",
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

fn html_crunched_number(slug:String) -> seed::dom_types::Node<Msg> {
    let tableStyle = style!{"border" => "1px #000 solid"};
    div![style!{"width" => "75%"; "padding" => "3px"},
        br![],
        br![],
        b!["The number you submitted to be crunched was:"],
        h1![slug.to_string()," - ",numerics_to_text::convert(slug.to_string())],
        table![attrs!{At::Class => "text", At::Width => "100%"}, &tableStyle,
            tbody![
                tr![
                    td![
                        "It is an odd number.",
                        br![],
                        "It is the 4th prime number.",
                        br![],
                        "It is the 2nd ",a!["mersenne prime", attrs!{At::Class => "link", At::Href => "http://en.wikipedia.org/wiki/Mersenne_prime"}],".",
                        br![],
                        "It is not a ",a!["fermat prime", attrs!{At::Class => "link", At::Href => "https://www.fermatsearch.org/"}],".",
                        br![],
                        "It is not a ",a!["perfect number", attrs!{At::Class => "link", At::Href => "https://en.wikipedia.org/wiki/Perfect_number"}],".",
                        br![],
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "text", At::Width => "100%"}, &tableStyle,
            tbody![
                tr![
                    td![
                        "It is not a triangle number.",
                        br![],
                        "It is not a square number.",
                        br![],
                        "It is not a cube number.",
                        br![],
                        br![],
                        "It is not a factorial number.",
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "text", At::Width => "100%"}, &tableStyle,
            tbody![
                tr![
                    td![
                        "It it has no factors except itself and 1.",
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "text", At::Width => "100%"}, &tableStyle,
            tbody![
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 2 (Binary):",
                    ],
                    td![
                        "111",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 3 (Ternary):",
                    ],
                    td![
                        "21",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 4 (Quaternary):",
                    ],
                    td![
                        "13",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 5 (Quinary):",
                    ],
                    td![
                        "12",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 8 (Octal):",
                    ],
                    td![
                        "7",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 10 (Denary):",
                    ],
                    td![
                        "7",
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 16 (Hexadecimal):",
                    ],
                    td![
                        "7",
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "text", At::Width => "100%"}, &tableStyle,
            tbody![
                tr![
                    td![attrs!{At::Width => "200"},
                        "Roman Numerals:",
                    ],
                    td![attrs!{At::Width => "40"},
                        "VII",
                    ],
                ],
                tr![
                    td![
                        "Egyptian Numerals:",
                    ],
                    td![
                        "𓐀",
                    ]
                ],
                tr![
                    td![
                        "Chinese Numerals:",
                    ],
                    td![style!{"vertical-align" => "middle"; "background-color" => "#FFF"},
                        "柒",
                    ]
                ],
                tr![
                    td![
                        "Babylonian Numerals:",
                    ],
                    td![style!{"vertical-align" => "middle"; "background-color" => "#FFF"},
                        img![attrs!{At::Src => "https://static.bigprimes.net/imgs/babnumbers/bab_7.gif", At::Alt => "7"}],
                    ]
                ],
            ],
        ],
    ]
}

pub fn render(slug:String) -> seed::dom_types::Node<Msg> {
    let rgx = Regex::new(r"^([1-9]+[0-9]*)$").unwrap();

    match rgx.is_match(&slug) {
        true => html_crunched_number(slug),
        _ => html_form()
    }
}
