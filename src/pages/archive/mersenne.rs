use seed::prelude::*;
use crate::Msg;

pub mod mersenne_utils {
    extern crate num_bigint;
    extern crate num_traits;
    
    use num_traits::{Pow,Num};
    use num_bigint::{BigInt,ToBigInt};

    pub fn mersennes_discovery_dates(n:usize) -> String {
        let mersennes_discovery_dates:Vec<Vec<&str>> = vec![
            //TODO: Complete this list all the way upto 50
            //TODO: We could include download links etc
            vec![""],//faux zero entry to make things easier when reading this vector
            vec!["500BC"],	
            vec!["500BC"],	
            vec!["275BC"],
            vec!["275BC"],
            vec!["1456"],	
            vec!["1588"],	
            vec!["1588"],	
            vec!["1772"],	
            vec!["1883"],	
            vec!["1911"],
            vec!["1914"],
            vec!["1876"],
            vec!["30-Jan-1952"],
            vec!["30-Jan-1952"],	
            vec!["26-Jun-1952"],
            vec!["7-Oct-1952"],
            vec!["9-Oct-1952"],
            vec!["8-Sep-1957"],		
            vec!["3-Nov-1961"],
            vec!["3-Nov-1961"],
            vec!["11-May-1963"],
            vec!["16-May-1963"],
            vec!["2-Jun-1963"],
            vec!["4-Mar-1971"],
            vec!["30-Oct-1978"],
            vec!["9-Feb-1979"],
            vec!["8-Apr-1979"],
            vec!["25-Sep-1982"],
            vec!["28-Jan-1988"],
            vec!["20-Sep-1983"],
            vec!["6-Sep-1985"],
            vec!["19-Feb-1992"],
            vec!["10-Jan-1994"],
            vec!["3-Sep-1996"],
            vec!["12-Nov-1996"],
            vec!["24-Aug-1997"],
            vec!["27-Jan-1998"],
            vec!["1-Jun-1999"],
            vec!["14-Nov-2001"],
            vec!["17-Nov-2003"],
            vec!["28-May-2004"],
            vec!["26-Feb-2005"],
            vec!["15-Dec-2005"],
            vec!["4-Sep-2006"],
            vec!["23-Aug-2008"],
            vec!["06-Sep-2009"],
            vec!["12-Apr-2008"],
            vec!["25-Jan-2013"],
            vec!["07-Jan-2016"],
            vec!["26-Dec-2017"],
        ];

        mersennes_discovery_dates[n][0].to_owned()
    }

    pub fn mersennes() -> Vec<Vec<usize>> {
        vec![
            //vec![p,digits]
            vec![0,     0],//faux zero entry to make things easier when reading this vector
            vec![2,     1],	
            vec![3,     1],	
            vec![5,     2],
            vec![7,     3],
            vec![13,    4],	
            vec![17,    6],
            vec![19,    6],	
            vec![31,   10],	
            vec![61,   19],	
            vec![89,   27],
            vec![107,  33],
            vec![127,  39],
            vec![521,  157],
            vec![607,  183],
            vec![1279, 386],
            vec![2203, 664],
            vec![2281, 687],
            vec![3217, 969],		
            vec![4253,1281],
            vec![4423,1332],
            vec![9689,2917],
            vec![9941,2993],
            vec![11213,3376],
            vec![19937,6002],
            vec![21701,6533],
            vec![23209,6987],
            vec![44497,13395],
            vec![86243,25962],
            vec![110503,33265],
            vec![132049,39751],
            vec![216091,65050],
            vec![756839,227832],
            vec![859433,258716],
            vec![1257787,378632],
            vec![1398269,420921],
            vec![2976221,895832],
            vec![3021377,909526],
            vec![6972593,2098960],
            vec![13466917,4053946],
            vec![20996011,6320430],
            vec![24036583,7235733],
            vec![25964951,7816230],
            vec![30402457,9152052],
            vec![32582657,9808358],
            vec![37156667,11185272],
            vec![42643801,12837064],
            vec![43112609,12978189],
            vec![57885161,17425170],
            vec![74207281,22338618],
            vec![77232917,23249425],
        ]
    }

    pub fn nth_mersenne_prime (candidate:&str) -> u64 {
        let big_candidate:BigInt = num_bigint::BigInt::from_str_radix(&candidate, 10).unwrap();
        let mersennes:Vec<Vec<usize>> = mersennes();

        let mut answer:u64 = 0;
        let big_two:BigInt = 2.to_bigint().unwrap();
        for n in 1..mersennes.len() {
            let mprime:BigInt = big_two.pow(mersennes[n][0]) - 1;
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

pub fn render() -> seed::dom_types::Node<Msg> {
    let mut html = vec![];

    let mersennes = mersenne_utils::mersennes();

    for n in 1..mersennes.len() {
        let download_txt:String = vec!["https://static.bigprimes.net/archive/mersenne/M",&n.to_string(),".txt"].into_iter().collect();
        let download_zip:String = vec!["https://static.bigprimes.net/archive/mersenne/M",&n.to_string(),".zip"].into_iter().collect();
        html.push(
            tr![
                td![n.to_string()],
                td!["2",sup![mersennes[n][0].to_string()],"-1"],
                td![mersennes[n][1].to_string()],
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
}