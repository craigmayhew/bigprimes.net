use crate::Msg;
use seed::prelude::*;

pub mod perfects_utils {
    use num_bigint::{BigUint, ToBigUint};
    use num_traits::ToPrimitive;

    #[derive(Clone)]
    pub struct Perfect {
        pub n: u64,
        pub p: u64,
        pub digits: u64,
        pub discovery: String,
    }

    #[derive(Clone)]
    pub struct PerfectDownload {
        pub n: u64,
        pub p: u64,
    }

    pub fn perfects() -> Vec<Perfect> {
        vec![
            Perfect {
                n: 51,
                p: 82589934,
                digits: 49724095,
                discovery: String::from("2018 Laroche, Woltman, Kurowski, Blosser, et al."),
            },
            Perfect {
                n: 50,
                p: 77232917,
                digits: 46498850,
                discovery: String::from(
                    "2017 Jonathan Pace, George Woltman, Scott Kurowski, Aaron Blosser, et al..",
                ),
            },
            Perfect {
                n: 49,
                p: 74207281,
                digits: 44677235,
                discovery: String::from("2016 Cooper, Woltman, Kurowski, Blosser et al."),
            },
            Perfect {
                n: 48,
                p: 57885161,
                digits: 34850340,
                discovery: String::from("2013 Cooper, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 47,
                p: 43112609,
                digits: 25956377,
                discovery: String::from("2008 Smith, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 46,
                p: 42643801,
                digits: 42643801,
                discovery: String::from("2009 Strindmo, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 45,
                p: 37156667,
                digits: 22370543,
                discovery: String::from("2008 Elvenich, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 44,
                p: 32582657,
                digits: 19616714,
                discovery: String::from("2006 Cooper, Boone, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 43,
                p: 30402457,
                digits: 18304103,
                discovery: String::from("2005 Cooper, Boone, Woltman, Kurowski, et al."),
            },
            Perfect {
                n: 42,
                p: 25964951,
                digits: 15632458,
                discovery: String::from("2005 Nowak, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 41,
                p: 24036583,
                digits: 14471465,
                discovery: String::from("2004 Findley, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 40,
                p: 20996011,
                digits: 12640858,
                discovery: String::from("2003 Shafer, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 39,
                p: 13466917,
                digits: 8107892,
                discovery: String::from("2001 Cameron, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 38,
                p: 6972593,
                digits: 4197919,
                discovery: String::from("1999 Hajratwala, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 37,
                p: 3021377,
                digits: 1819050,
                discovery: String::from("1998 Clarkson, Woltman, Kurowski, et. al."),
            },
            Perfect {
                n: 36,
                p: 2976221,
                digits: 1791864,
                discovery: String::from("1997 Spence, Woltman, et. al."),
            },
            Perfect {
                n: 35,
                p: 1398269,
                digits: 841842,
                discovery: String::from("1996 Armengaud, Woltman, et. al."),
            },
            Perfect {
                n: 34,
                p: 1257787,
                digits: 757263,
                discovery: String::from("1996 Slowinski & Gage"),
            },
            Perfect {
                n: 33,
                p: 859433,
                digits: 517430,
                discovery: String::from("1994 Slowinski & Gage"),
            },
            Perfect {
                n: 32,
                p: 756839,
                digits: 455663,
                discovery: String::from("1992 Slowinski & Gage"),
            },
            Perfect {
                n: 31,
                p: 216091,
                digits: 130100,
                discovery: String::from("1985 Slowinski"),
            },
            Perfect {
                n: 30,
                p: 132049,
                digits: 79502,
                discovery: String::from("1983 Slowinski"),
            },
            Perfect {
                n: 29,
                p: 110503,
                digits: 66530,
                discovery: String::from("1988 Colquitt & Welsh"),
            },
            Perfect {
                n: 28,
                p: 86243,
                digits: 51924,
                discovery: String::from("1982 Slowinski"),
            },
            Perfect {
                n: 27,
                p: 44497,
                digits: 26790,
                discovery: String::from("1979 Nelson & Slowinski"),
            },
            Perfect {
                n: 26,
                p: 23209,
                digits: 13973,
                discovery: String::from("1979 Noll"),
            },
            Perfect {
                n: 25,
                p: 21701,
                digits: 13066,
                discovery: String::from("1978 Noll & Nickel"),
            },
            Perfect {
                n: 24,
                p: 19937,
                digits: 12003,
                discovery: String::from("1971 Tuckerman"),
            },
            Perfect {
                n: 23,
                p: 11213,
                digits: 6751,
                discovery: String::from("1963 Gillies"),
            },
            Perfect {
                n: 22,
                p: 9941,
                digits: 5985,
                discovery: String::from("1963 Gillies"),
            },
            Perfect {
                n: 21,
                p: 9689,
                digits: 5834,
                discovery: String::from("1963 Gillies"),
            },
            Perfect {
                n: 20,
                p: 4423,
                digits: 2663,
                discovery: String::from("1961 Hurwitz"),
            },
            Perfect {
                n: 19,
                p: 4253,
                digits: 2561,
                discovery: String::from("1961 Hurwitz"),
            },
            Perfect {
                n: 18,
                p: 3217,
                digits: 1937,
                discovery: String::from("1957 Riesel"),
            },
            Perfect {
                n: 17,
                p: 2281,
                digits: 1373,
                discovery: String::from("1952 Robinson"),
            },
            Perfect {
                n: 16,
                p: 2203,
                digits: 1327,
                discovery: String::from("1952 Robinson"),
            },
            Perfect {
                n: 15,
                p: 1279,
                digits: 770,
                discovery: String::from("1952 Robinson"),
            },
            Perfect {
                n: 14,
                p: 607,
                digits: 366,
                discovery: String::from("1952 Robinson"),
            },
            Perfect {
                n: 13,
                p: 521,
                digits: 314,
                discovery: String::from("1952 Robinson"),
            },
            Perfect {
                n: 12,
                p: 127,
                digits: 77,
                discovery: String::from("1876 Lucas"),
            },
            Perfect {
                n: 11,
                p: 107,
                digits: 65,
                discovery: String::from("1914 Powers"),
            },
            Perfect {
                n: 10,
                p: 89,
                digits: 54,
                discovery: String::from("1911 Powers"),
            },
            Perfect {
                n: 9,
                p: 61,
                digits: 37,
                discovery: String::from("1883 Pervushin"),
            },
            Perfect {
                n: 8,
                p: 31,
                digits: 19,
                discovery: String::from("1772 Euler"),
            },
            Perfect {
                n: 7,
                p: 19,
                digits: 12,
                discovery: String::from("1588 Cataldi"),
            },
            Perfect {
                n: 6,
                p: 17,
                digits: 10,
                discovery: String::from("1588 Cataldi"),
            },
            Perfect {
                n: 5,
                p: 13,
                digits: 8,
                discovery: String::from("1456 ?"),
            },
            Perfect {
                n: 4,
                p: 7,
                digits: 4,
                discovery: String::from("?"),
            },
            Perfect {
                n: 3,
                p: 5,
                digits: 3,
                discovery: String::from("?"),
            },
            Perfect {
                n: 2,
                p: 3,
                digits: 2,
                discovery: String::from("?"),
            },
            Perfect {
                n: 1,
                p: 2,
                digits: 1,
                discovery: String::from("?"),
            },
        ]
    }

    pub fn equation(p: u64) -> BigUint {
        let two: BigUint = 2.to_biguint().unwrap();
        let power: BigUint = two.clone() << (p.to_usize().unwrap() - 1 - 1);
        power.clone() * ((power.clone() * two.clone()) - 1.to_biguint().unwrap())
    }
}

pub fn render(model: &crate::Model) -> Node<Msg> {
    let mut html = vec![];
    let perfects = perfects_utils::perfects().to_owned();

    for perfect in perfects.iter() {
        let perfectp: u64 = perfect.p;
        let perfect_download = perfects_utils::PerfectDownload {
            n: perfect.n,
            p: perfect.p,
        };
        let equation: String = format!(
            "2<sup>{}</sup> × (2<sup>{}</sup>-1)",
            &(perfectp - 1).to_string(),
            &(perfectp).to_string()
        );

        html.push(tr![
            td![perfect.n.to_string()],      //rank
            td![El::from_html(&equation)],   //perfect number as a formula
            td![perfect.digits.to_string()], //digits in length
            td![perfect.discovery.to_string()],          //discovery
            if model.download.n == perfect.n {
                td![crate::utils::generate_file(
                    model.download.n,
                    perfects_utils::equation(model.download.p)
                )]
            } else {
                td![button![
                    "Generate",
                    mouse_ev("mouseup", move |event| Msg::GeneratePerfectDownload(
                        event,
                        perfect_download
                    ))
                ]]
            },
        ]);
    }

    div![
        h1!["The Perfect Numbers"],
        br![],
        br![],
        br![],
        table![
            attrs! {At::Class => "perfecttable text"},
            tbody![
                tr![
                    td![b!["No."]],
                    td![b!["Prime"]],
                    td![b!["Digits"]],
                    td![b!["Discovered"]],
                    td![b!["Download"]]
                ],
                html
            ]
        ]
    ]
}

#[cfg(test)]
use num_bigint::BigUint;
#[cfg(test)]
use num_traits::Num;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_test<'a>() {
        let mut perfects: Vec<perfects_utils::Perfect> = perfects_utils::perfects();
        perfects.reverse();
        assert_eq!(perfects[50].digits, 49724095);
        assert_eq!(perfects[49].digits, 46498850);
        assert_eq!(perfects[39].digits, 12640858);
    }

    #[test]
    fn equation_test() {
        let mut number: BigUint;

        number = num_bigint::BigUint::from_str_radix("6", 10).unwrap();
        assert_eq!(perfects_utils::equation(2), number);

        number = num_bigint::BigUint::from_str_radix("137438691328", 10).unwrap();
        assert_eq!(perfects_utils::equation(19), number);

        number = num_bigint::BigUint::from_str_radix("2658455991569831744654692615953842176", 10)
            .unwrap();
        assert_eq!(perfects_utils::equation(61), number);
    }
}
