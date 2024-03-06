use crate::Msg;
use seed::prelude::*;

extern crate test;

/// # fns for generating Fermats
mod fermat_utils {
    use num_traits::One;

    /// # Fermat struct
    pub struct Fermat<'a> {
        pub n: u64,
        pub f: u64,
        pub digits: u64,
        pub prime_factors: &'a str,
    }

    impl<'a> Fermat<'a> {
        /// # Return a Vec<String> from a Fermat struct
        ///
        /// Implement string conversion for each field
        pub fn to_vec(&self) -> Vec<String> {
            use num_bigint::{BigUint, ToBigUint};
            use num_traits::pow;

            let two_to_the_n = 1 << self.n;

            let equation: String = format!("2<sup>{}</sup>+1", &two_to_the_n);

            let download_filename: String = format!("F{}.txt", &self.n);

            let fermat_value: BigUint =
                pow(2_u32.to_biguint().unwrap(), two_to_the_n) + BigUint::one();

            vec![
                self.n.to_string(),
                equation,
                self.digits.to_string(),
                self.prime_factors.to_string(),
                download_filename,
                fermat_value.to_string(),
            ]
        }
    }

    /// # Return Vec of populated Fermat structs
    ///
    /// The data to populate the Fermat structs is hard coded within this fn
    pub fn fermats<'a>() -> Vec<Fermat<'a>> {
        vec![
			Fermat {n: 11, f: 0, digits:  617, prime_factors: "<a href=\"/cruncher/319489/\">P27567</a> × 974849 × 167988556341760475137 × 3560841906445833920513 × <a href=\"/cruncher/4093/\">P564</a>"},
            Fermat {n: 10, f: 0, digits:  309, prime_factors: "45592577 × 6487031809 × 4659775785220018543264560743076778192897 × <a href=\"/cruncher/1601/\">P252</a>"},
            Fermat {n:  9, f: 0, digits:  155, prime_factors: "2424833 × 7455602825647884208337395736200454918783366342657 × <a href=\"/cruncher/523/\">P99</a>"},
            Fermat {n:  8, f: 0, digits:   78, prime_factors: "1238926361552897 × <a href=\"/cruncher/293/\">P62</a>"},
            Fermat {n:  7, f: 0, digits:   39, prime_factors: "59649589127497217 × 5704689200685129054721"},
            Fermat {n:  6, f: 0, digits:   20, prime_factors: "274177 × 67280421310721"},
            Fermat {n:  5, f: 0, digits:   10, prime_factors: "<a href=\"/cruncher/641/\">P116</a> × 6700417"},
            Fermat {n:  4, f: 0, digits:    5, prime_factors: "<a href=\"/cruncher/65537/\">P6543</a>"},
            Fermat {n:  3, f: 0, digits:    3, prime_factors: "<a href=\"/cruncher/257/\">P55</a>"},
            Fermat {n:  2, f: 0, digits:    2, prime_factors: "<a href=\"/cruncher/17/\">P7</a>"},
            Fermat {n:  1, f: 0, digits:    1, prime_factors: "<a href=\"/cruncher/5/\">P3</a>"},
            Fermat {n:  0, f: 0, digits:    1, prime_factors: "<a href=\"/cruncher/3/\">P2</a>"},
        ]
    }
}

/// # Return html table rows populated with Fermats
fn generate_rows() -> std::vec::Vec<Node<Msg>> {
    let mut html = Vec::new();

    let fermats = fermat_utils::fermats();

    for f_n in fermats {
        let f = f_n.to_vec();

        html.push(tr![
            td![&f[0]],                      //rank
            td![El::from_html(None, &f[1])], //fermat number as a formula
            td![&f[2]],                      //digits in length
            td![El::from_html(None, &f[3])], //prime factors
            td![crate::utils::save_as_file(
                f[4].to_string(),
                f[5].to_string()
            )],
        ]);
    }

    html
}

/// # Generate html using seed macros
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
    use test::Bencher;

    #[test]
    fn fermat_test<'a>() {
        let mut fermats: Vec<fermat_utils::Fermat<'a>> = fermat_utils::fermats();
        fermats.reverse();
        assert_eq!(fermats[2].prime_factors, "<a href=\"/cruncher/17/\">P7</a>");
        assert_eq!(fermats[11].digits, 617);
    }

    #[bench]
    fn fermat_bench(b: &mut Bencher) {
        b.iter(|| fermat_utils::fermats());
    }

    #[test]
    fn fermat_to_vec_test<'a>() {
        let fermats = fermat_utils::fermats();
        assert_eq!(
            fermats[9].to_vec()[3],
            "<a href=\"/cruncher/17/\">P7</a>".to_string()
        );
        assert_eq!(fermats[0].to_vec()[2], 617.to_string());
        assert_eq!(fermats[3].to_vec()[1], "2<sup>256</sup>+1".to_string());
    }

    #[bench]
    fn fermat_to_vec_bench(b: &mut Bencher) {
        b.iter(|| {
            fermat_utils::fermats()
                .iter()
                .any(|v| !v.to_vec().is_empty())
        });
    }
}
