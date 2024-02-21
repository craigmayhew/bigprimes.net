use crate::Msg;
use seed::prelude::*;
use std::convert::TryInto;

use num_bigint::{BigUint, ToBigUint};
use num_traits::pow;

mod fermat_utils {
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
}

fn generate_rows() -> std::vec::Vec<Node<Msg>> {
    let mut html = Vec::new();
    let two: usize = 2;

    let fermats = fermat_utils::fermats();

    for n in 0..fermats.len() {
        let download_filename: String = format!("F{}.txt", &fermats[n].n.to_string());

        let equation: String = format!(
            "2<sup>{}</sup>+1",
            &(two.pow((fermats.len() - 1 - n).try_into().unwrap())).to_string()
        );
        let nth_fermat: usize = fermats.len() - 1 - n; //counts from 11 to 0
        let fermat_value: BigUint =
            pow(two.to_biguint().unwrap(), pow(two, nth_fermat)) + 1.to_biguint().unwrap();

        html.push(tr![
            td![fermats[n].n.to_string()],                      //rank
            td![El::from_html(None, &equation)],                //fermat number as a formula
            td![fermats[n].digits.to_string()],                 //digits in length
            td![El::from_html(None, fermats[n].prime_factors)], //prime factors
            td![crate::utils::save_as_file(
                download_filename,
                fermat_value.to_string()
            )],
        ]);
    }

    html
}

pub fn render() -> Node<Msg> {
    let html: std::vec::Vec<Node<Msg>> = generate_rows();

    div![
        h1!["The Fermat Numbers"],
        br![],
        br![],
        br![],
        table![
            attrs! {At::Id => "tbl"},
            tbody![
                tr![
                    td![b!["No."]],
                    td![b!["Fermat"]],
                    td![b!["Digits"]],
                    td![b!["Prime Factors"]],
                    td![b!["Download"]]
                ],
                html
            ]
        ]
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fermat_test<'a>() {
        let mut fermats: Vec<fermat_utils::Fermat<'a>> = fermat_utils::fermats();
        fermats.reverse();
        assert_eq!(fermats[2].prime_factors, "<a href=\"/cruncher/17/\">P7</a>");
        assert_eq!(fermats[11].digits, 617);
    }
}
