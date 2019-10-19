use seed::prelude::*;
use crate::Msg;

pub mod mersenne_utils {
    extern crate num_bigint;
    extern crate num_traits;
    
    use num_traits::{Pow,Num};
    use num_bigint::{BigInt,ToBigInt};

    #[derive(Clone)]
	pub struct Mersenne {
		pub n: u64,
		pub p: u64,
		pub digits: u64,
		pub discovery: String,
	}

    pub fn mersennes_discovery_dates(n:usize) -> String {
        let mersennes_discovery_dates:Vec<Mersenne> = mersennes();

        mersennes_discovery_dates[n].discovery.to_owned()
    }

    pub fn mersennes() -> Vec<Mersenne> {
        vec![
            //vec![p,digits]
            Mersenne {n: 0, p: 0, digits: 0, discovery: String::from("")},//faux zero entry to make things easier when reading this vector
            Mersenne {n: 1, p: 2, digits: 1, discovery: String::from("500BC")},
            Mersenne {n: 2, p: 3, digits: 1, discovery: String::from("500BC")},
            Mersenne {n: 3, p: 5, digits: 2, discovery: String::from("275BC")},
            Mersenne {n: 4, p: 7, digits: 3, discovery: String::from("275BC")},
            Mersenne {n: 5, p: 13, digits: 4, discovery: String::from("1456")},
            Mersenne {n: 6, p: 17, digits: 6, discovery: String::from("1588")},
            Mersenne {n: 7, p: 19, digits: 6, discovery: String::from("1588")},
            Mersenne {n: 8, p: 31, digits: 10, discovery: String::from("1772")},
            Mersenne {n: 9, p: 61, digits: 19, discovery: String::from("1883")},
            Mersenne {n: 10, p: 89, digits: 27, discovery: String::from("1911")},
            Mersenne {n: 11, p: 107, digits: 33, discovery: String::from("1914")},
            Mersenne {n: 12, p: 127, digits: 39, discovery: String::from("1876")},
            Mersenne {n: 13, p: 521, digits: 157, discovery: String::from("30-Jan-1952")},
            Mersenne {n: 14, p: 607, digits: 183, discovery: String::from("30-Jan-1952")},
            Mersenne {n: 15, p: 1279, digits: 386, discovery: String::from("26-Jun-1952")},
            Mersenne {n: 16, p: 2203, digits: 664, discovery: String::from(" 7-Oct-1952")},
            Mersenne {n: 17, p: 2281, digits: 687, discovery: String::from(" 9-Oct-1952")},
            Mersenne {n: 18, p: 3217, digits: 969, discovery: String::from(" 8-Sep-1957")},
            Mersenne {n: 19, p: 4253, digits: 1281, discovery: String::from(" 3-Nov-1961")},
            Mersenne {n: 20, p: 4423, digits: 1332, discovery: String::from(" 3-Nov-1961")},
            Mersenne {n: 21, p: 9689, digits: 2917, discovery: String::from("11-May-1963")},
            Mersenne {n: 22, p: 9941, digits: 2993, discovery: String::from("16-May-1963")},
            Mersenne {n: 23, p: 11213, digits: 3376, discovery: String::from(" 2-Jun-1963")},
            Mersenne {n: 24, p: 19937, digits: 6002, discovery: String::from(" 4-Mar-1971")},
            Mersenne {n: 25, p: 21701, digits: 6533, discovery: String::from("30-Oct-1978")},
            Mersenne {n: 26, p: 23209, digits: 6987, discovery: String::from(" 9-Feb-1979")},
            Mersenne {n: 27, p: 44497, digits: 13395, discovery: String::from(" 8-Apr-1979")},
            Mersenne {n: 28, p: 86243, digits: 25962, discovery: String::from("25-Sep-1982")},
            Mersenne {n: 29, p: 110503, digits: 33265, discovery: String::from("28-Jan-1988")},
            Mersenne {n: 30, p: 132049, digits: 39751, discovery: String::from("20-Sep-1983")},
            Mersenne {n: 31, p: 216091, digits: 65050, discovery: String::from(" 6-Sep-1985")},
            Mersenne {n: 32, p: 756839, digits: 227832, discovery: String::from("19-Feb-1992")},
            Mersenne {n: 33, p: 859433, digits: 258716, discovery: String::from("10-Jan-1994")},
            Mersenne {n: 34, p: 1257787, digits: 378632, discovery: String::from(" 3-Sep-1996")},
            Mersenne {n: 35, p: 1398269, digits: 420921, discovery: String::from("12-Nov-1996")},
            Mersenne {n: 36, p: 2976221, digits: 895832, discovery: String::from("24-Aug-1997")},
            Mersenne {n: 37, p: 3021377, digits: 909526, discovery: String::from("27-Jan-1998")},
            Mersenne {n: 38, p: 6972593, digits: 2098960, discovery: String::from(" 1-Jun-1999")},
            Mersenne {n: 39, p: 13466917, digits: 4053946, discovery: String::from("14-Nov-2001")},
            Mersenne {n: 40, p: 20996011, digits: 6320430, discovery: String::from("17-Nov-2003")},
            Mersenne {n: 41, p: 24036583, digits: 7235733, discovery: String::from("28-May-2004")},
            Mersenne {n: 42, p: 25964951, digits: 7816230, discovery: String::from("26-Feb-2005")},
            Mersenne {n: 43, p: 30402457, digits: 9152052, discovery: String::from("15-Dec-2005")},
            Mersenne {n: 44, p: 32582657, digits: 9808358, discovery: String::from(" 4-Sep-2006")},
            Mersenne {n: 45, p: 37156667, digits: 11185272, discovery: String::from("23-Aug-2008")},
            Mersenne {n: 46, p: 42643801, digits: 12837064, discovery: String::from("06-Sep-2009")},
            Mersenne {n: 47, p: 43112609, digits: 12978189, discovery: String::from("12-Apr-2008")},
            Mersenne {n: 48, p: 57885161, digits: 17425170, discovery: String::from("25-Jan-2013")},
            Mersenne {n: 49, p: 74207281, digits: 22338618, discovery: String::from("07-Jan-2016")},
            Mersenne {n: 50, p: 77232917, digits: 23249425, discovery: String::from("26-Dec-2017")},
        ]
    }

    pub fn nth_mersenne_prime (candidate:&str) -> u64 {
        let big_candidate:BigInt = num_bigint::BigInt::from_str_radix(&candidate, 10).unwrap();
        let mersennes:Vec<Mersenne> = mersennes();

        let mut answer:u64 = 0;
        let big_two:BigInt = 2.to_bigint().unwrap();
        for n in 1..mersennes.len() {
            let mprime:BigInt = big_two.pow(mersennes[n].p) - 1;
            if big_candidate == mprime {
                answer = n as u64;
                break
            } else if big_candidate < mprime {
                break
            }
        }

        answer
    }
}

pub fn render(model: &crate::Model) -> seed::dom_types::Node<Msg> {
    let mut html = vec![];
    let mersennes = mersenne_utils::mersennes();

    for n in 1..mersennes.len() {
        let download_txt:String = format!("https://static.bigprimes.net/archive/mersenne/M{}.txt",&n.to_string());
        let download_zip:String = format!("https://static.bigprimes.net/archive/mersenne/M{}.zip",&n.to_string());
        html.push(
            tr![
                td![n.to_string()],
                td!["2",sup![mersennes[n].p.to_string()],"-1"],
                td![mersennes[n].digits.to_string()],
                td![mersenne_utils::mersennes_discovery_dates(n)],
                if n >= 30 {a![attrs!{At::Href => download_zip},"ZIP"]} else {a![attrs!{At::Href => download_txt},"TXT"]}
            ]
        );
    }

    html.reverse();

    div![
        h1!["The Mersenne Numbers"],
        br![],
        br![],
        br![],
        table![
            attrs!{At::Class => "mersennetable text"},
            tbody![
                tr![
                    td![
                        b!["No."]
                    ],
                    td![
                        b!["Prime"]
                    ],	
                    td![
                        b!["Digits"]
                    ],	
                    td![
                        b!["Discovered"]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mersennes_discovery_dates_test() {
        assert_eq!(mersenne_utils::mersennes_discovery_dates(0), "");
        assert_eq!(mersenne_utils::mersennes_discovery_dates(50), "26-Dec-2017");
    }

    #[test]
    fn nth_mersenne_prime_test() {
        assert_eq!(mersenne_utils::nth_mersenne_prime("127"), 4);
        assert_eq!(mersenne_utils::nth_mersenne_prime("9"), 0);
    }

    #[test]
    fn mersenne_test() {
        let mersennes:Vec<mersenne_utils::Mersenne> = mersenne_utils::mersennes();
        assert_eq!(mersennes[0].p, 0);
        assert_eq!(mersennes[0].digits, 0);

        assert_eq!(mersennes[2].p, 3);
        assert_eq!(mersennes[2].digits, 1);
    }
}