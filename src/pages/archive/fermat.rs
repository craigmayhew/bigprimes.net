use seed::prelude::*;
use crate::Msg;
use std::convert::TryInto;

mod fermat_utils {
    use seed::prelude::*;
    use crate::Msg;

    pub struct Fermat<'a> {
		pub n: u64,
		pub f: u64,
		pub digits: u64,
		pub prime_factors: &'a str,
        pub download: &'a str,
	}

    pub fn fermats<'a>() -> Vec<Fermat<'a>> {
        vec![
			Fermat {n: 11, f: 0, digits:  617, prime_factors: "<a href=\"/cruncher/319489/\">P27567</a> × 974849 × 167988556341760475137 × 3560841906445833920513 × <a href=\"/cruncher/4093/\">P564</a>", download: "F0"},
            Fermat {n: 10, f: 0, digits:  309, prime_factors: "45592577 × 6487031809 × 4659775785220018543264560743076778192897 × <a href=\"/cruncher/1601/\">P252</a>", download: "F0"},
            Fermat {n:  9, f: 0, digits:  155, prime_factors: "2424833 × 7455602825647884208337395736200454918783366342657 × <a href=\"/cruncher/523/\">P99</a>", download: "F0"},
            Fermat {n:  8, f: 0, digits:   78, prime_factors: "1238926361552897 × <a href=\"/cruncher/293/\">P62</a>", download: "F0"},
            Fermat {n:  7, f: 0, digits:   39, prime_factors: "59649589127497217 × 5704689200685129054721", download: "F0"},
            Fermat {n:  6, f: 0, digits:   20, prime_factors: "274177 × 67280421310721", download: "F0"},
            Fermat {n:  5, f: 0, digits:   10, prime_factors: "<a href=\"/cruncher/641/\">P116</a> × 6700417", download: "F0"},
            Fermat {n:  4, f: 0, digits:    5, prime_factors: "<a href=\"/cruncher/65537/\">P6543</a>", download: "F0"},
            Fermat {n:  3, f: 0, digits:    3, prime_factors: "<a href=\"/cruncher/257/\">P55</a>", download: "F0"},
            Fermat {n:  2, f: 0, digits:    2, prime_factors: "<a href=\"/cruncher/17/\">P7</a>", download: "F0"},
            Fermat {n:  1, f: 0, digits:    1, prime_factors: "<a href=\"/cruncher/5/\">P3</a>", download: "F0"},
            Fermat {n:  0, f: 0, digits:    1, prime_factors: "<a href=\"/cruncher/3/\">P2</a>", download: "F0"},
        ]
    }

    pub fn save_as_file(filename:String, filecontent:String) -> seed::dom_types::Node<Msg> {
        let href:String = vec!["data:text/plain;",&filecontent].into_iter().collect();
        a![attrs!{At::Download => &filename, At::Href => &href}, "TXT"]
    }
}

pub fn render() -> seed::dom_types::Node<Msg> {
    let mut html = vec![];
    let two:usize = 2;

    let mut fermats = fermat_utils::fermats();

    for n in 0..fermats.len() {
        let download_txt:String = vec!["https://static.bigprimes.net/archive/fermat/F",&fermats[n].n.to_string(),".txt"].into_iter().collect();

		let equation:String = vec!["2<sup>",&(two.pow((fermats.len()-1-n).try_into().unwrap())).to_string(),"</sup>+1"].into_iter().collect();

        html.push(
            tr![
                td![fermats[n].n.to_string()],//rank
				td![El::from_html(&equation)],//fermat number as a formula
				
                td![fermats[n].digits.to_string()],//digits in length
                td![El::from_html(fermats[n].prime_factors)],//prime factors
                a![attrs!{At::Href => download_txt},"TXT"],//downloads
            ]
        );
    }

    fermats.reverse();
    
    div![
        h1!["The Fermat Numbers"],
        br![],
        br![],
        br![],
        table![
            attrs!{At::Id => "tbl"},
            tbody![
                tr![
                    td![
                        b!["No."]
                    ],
                    td![
                        b!["Fermat"]
                    ],	
                    td![
                        b!["Digits"]
                    ],	
                    td![
                        b!["Prime Factors"]
                    ],
                    td![
                        b!["Download"]
                    ]
                ],
                html
            ]
        ]
    ]
}