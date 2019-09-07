use seed::prelude::*;
use crate::Msg;

extern crate num_bigint as bigint;
extern crate num_traits;

use crate::utils::{nth};
use crate::pages::archive::mersenne::mersenne_utils as mersenne;
use crate::pages::archive::prime::prime as prime;
use regex::Regex;

const MAX_LEN_PRIME_CHECK: usize = 2;

mod numerics_to_text {
    use crate::utils::{nth};
    use num_traits::{Num,ToPrimitive,Zero,One,pow};
    use num_bigint::{BigUint,ToBigUint};

    pub fn is_odd(str_num: &str) -> bool{
        match str_num[str_num.len()-1..str_num.len()].as_ref() {
           "1"|"3"|"5"|"7"|"9" => true,
           _ => false,
        }
    }

    fn den2numerals(n:&str, glyphs:Vec<Vec<&str>>) -> String {
        //this function converts a number in the form of a string
        //to roman/egyptian/babylonian/chinese numerals
        let mut numerals:String = "".to_owned();
        let n_vec:Vec<char> = n.chars().collect();
        for i in 0..n.len() {
            let digit:String = n_vec[n.len()-1-i].to_string();
            if digit != "0" {
                let digit_usize:usize = digit.parse().unwrap();
                let numerals_tmp:String = numerals;
                numerals = glyphs[i][digit_usize-1].to_owned();
                numerals.push_str(&numerals_tmp);
            }
        }
        numerals
    }

    pub fn den_to_babylonian(str_num:&str) -> String {
        let mut glyphs:Vec<String> = vec!["".to_owned(); 60];
        glyphs[0] = " &nbsp; &nbsp; &nbsp; ".to_owned();
        for i in 1..59 {
            //TODO: replace these with unicode or svgs
            //e.g. if the license allows, replace with https://commons.wikimedia.org/wiki/File:Babylonian_numerals.svg
            glyphs[i].push_str("<img height=\"15\" src=\"https://static.bigprimes.net/imgs/babnumbers/bab_");
            glyphs[i].push_str(&i.to_string());
            glyphs[i].push_str(".gif\" alt=\"");
            glyphs[i].push_str(&i.to_string());
            glyphs[i].push_str("\">");
        }
        let mut val:Vec<&str> = vec![""; 1000];
        let mut num:BigUint = num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap();
        let sixty:BigUint = 60.to_biguint().unwrap();
        while num > Zero::zero() {
            val.push(&glyphs[(&num % &sixty).to_usize().unwrap()]);
            val.push(" &nbsp; ");
            num /= &sixty;
        }
        //reverse the vector
        val = val.iter().rev().cloned().collect();
        //convert to String
        val.into_iter().collect()
        
    }

    pub fn dec_to_base(str_num:&str, base:u32) -> String {
        num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap().to_str_radix(base)
    }

    pub fn den_to_chinese(n:&str) -> String {
        let glyphs:Vec<Vec<&str>> = vec![
            vec![ //units
                "&#22777;",
                "&#36019;",
                "&#21444;",
                "&#32902;",
                "&#20237;",
                "&#38520;",
                "&#26578;",
                "&#25420;",
                "&#29590;"
            ],
            vec![ //tens
                "&#22777;&#25342;",
                "&#36019;&#25342;",
                "&#21444;&#25342;",
                "&#32902;&#25342;",
                "&#20237;&#25342;",
                "&#38520;&#25342;",
                "&#26578;&#25342;",
                "&#25420;&#25342;",
                "&#29590;&#25342;"
            ],
            vec![ //hundreds
                "&#22777;&#20336;",
                "&#36019;&#20336;",
                "&#21444;&#20336;",
                "&#32902;&#20336;",
                "&#20237;&#20336;",
                "&#38520;&#20336;",
                "&#26578;&#20336;",
                "&#25420;&#20336;",
                "&#29590;&#20336;"
            ],
            vec![ //thousands
                "&#22777;&#20191;",
                "&#36019;&#20191;",
                "&#21444;&#20191;",
                "&#32902;&#20191;",
                "&#20237;&#20191;",
                "&#38520;&#20191;",
                "&#26578;&#20191;",
                "&#25420;&#20191;",
                "&#29590;&#20191;"
            ],
            vec![ //tenthousands
                "&#22777;&#33836;",
                "&#36019;&#33836;",
                "&#21444;&#33836;",
                "&#32902;&#33836;",
                "&#20237;&#33836;",
                "&#38520;&#33836;",
                "&#26578;&#33836;",
                "&#25420;&#33836;",
                "&#29590;&#33836;"
            ],
            vec![ //hundred thousands
                "&#22777;&#25342;&#33836;",
                "&#36019;&#25342;&#33836;",
                "&#21444;&#25342;&#33836;",
                "&#32902;&#25342;&#33836;",
                "&#20237;&#25342;&#33836;",
                "&#38520;&#25342;&#33836;",
                "&#26578;&#25342;&#33836;",
                "&#25420;&#25342;&#33836;",
                "&#29590;&#25342;&#33836;"
            ],
        ];

        den2numerals(&n, glyphs)
    }

    pub fn den_to_egyptian(n:&str) -> String {
        let glyphs:Vec<Vec<&str>> = vec![
            vec![
                "&#x133fa;",
                "&#x133fb;",
                "&#x133fc;",
                "&#x133fd;",
                "&#x133fe;",
                "&#x133ff;",
                "&#x13400;",
                "&#x13401;",
                "&#x13402;",
            ],
            vec![
                "&#x13386;",
                "&#x13387;",
                "&#x13388;",
                "&#x13389;",
                "&#x1338a;",
                "&#x1338b;",
                "&#x1338c;",
                "&#x1338d;",
                "&#x1338e;",
            ],
            vec![
                "&#x13362;",
                "&#x13363;",
                "&#x13364;",
                "&#x13365;",
                "&#x13366;",
                "&#x13367;",
                "&#x13368;",
                "&#x13369;",
                "&#x1336a;",
            ],
            vec![
                "&#x131bc;",
                "&#x131bd;",
                "&#x131be;",
                "&#x131bf;",
                "&#x131c0;",
                "&#x131c1;",
                "&#x131c2;",
                "&#x131c3;",
                "&#x131c4;",
            ],
            vec![
                "&#x130ad;",
                "&#x130ae;",
                "&#x130af;",
                "&#x130b0;",
                "&#x130b1;",
                "&#x130b2;",
                "&#x130b3;",
                "&#x130b4;",
                "&#x130b5;",
            ],
            vec![
                "&#x13190;",
                "&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;",
                "&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;&#x13190;",
            ],
            vec![
                "&#x13068;",
                "&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;",
                "&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;&#x13068;",
            ],
        ];

        den2numerals(&n, glyphs)
    }

    pub fn den_to_roman(n:&str) -> String {
        let glyphs:Vec<Vec<&str>> = vec![
            vec![ //units
                "I",
                "II",
                "III",
                "IV",
                "V",
                "VI",
                "VII",
                "VIII",
                "IX",
                "X"
            ],
            vec![ //tens
                "X",
                "XX",
                "XXX",
                "XL",
                "L",
                "LX",
                "LXX",
                "LXXX",
                "XC",
                "C"
            ],
            vec![ //hundreds
                "C",
                "CC",
                "CCC",
                "CD",
                "D",
                "DC",
                "DCC",
                "DCCC",
                "CM",
                "M"
            ],
            vec![ //THOUSANDS
                "M",
                "MM",
                "MMM",
                "M<u>V</u>",
                "<u>V</u>",
                "<u>V</u>M",
                "<u>V</u>MM",
                "<u>V</u>MMM",
                "M<u>X</u>",
                "<u>X</u>"
            ],
            vec![ //TEN THOUSANDS
                "<u>X</u>",
                "<u>X</u><u>X</u>",
                "<u>X</u><u>X</u><u>X</u>",
                "<u>X</u><span class=\"u\">L</span>",
                "<span class=\"u\">L</span>",
                "<span class=\"u\">L</span><u>X</u>",
                "<span class=\"u\">L</span><u>X</u><u>X</u>",
                "<span class=\"u\">L</span><u>X</u><u>X</u><u>X</u>",
                "<u>X</u><u>C</u>",
                "<u>C</u>"
            ],
            vec![ //HUNDRED THOUSANDS
                "<u>C</u>",
                "<u>C</u><u>C</u>",
                "<u>C</u><u>C</u><u>C</u>",
                "<u>C</u><u>D</u>",
                "<u>D</u>",
                "<u>D</u><u>C</u>",
                "<u>D</u><u>C</u><u>C</u>",
                "<u>D</u><u>C</u><u>D</u><u>C</u>",
                "<u>C</u><u>M</u>",
                "<u>M</u>"
            ]
        ];

        den2numerals(&n, glyphs)
    }

    pub fn convert(str_num: String) -> String {
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
        ];
        
        if str_num.len() > triplets.len()*3 {
            "Error".to_string()
        } else {
            let num:BigUint = num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap();
            let string:String = convert_tri(num, Zero::zero(), units, tens, triplets);
            //remove first character which is a space
            string[1..].to_string()
        }
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
            string.push_str(units[(y % 10)]);
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

    fn factor(n: u64) -> Vec<u64> {
        let mut factors: Vec<u64> = Vec::new(); // creates a new vector for the factors of the number
    
        for i in 1..((n as f64).sqrt() as u64 + 1) { 
            if n % i == 0 {
                factors.push(i); // pushes smallest factor to factors
                factors.push(n/i); // pushes largest factor to factors
            }
        }
        factors.sort(); // sorts the factors into numerical order for viewing purposes
        factors // returns the factors
    }

    pub fn list_factors(str_num: &str, glue: String) -> String {
        let num:u64 = str_num.parse().unwrap();
        let factors:Vec<u64> = factor(num);

        //convert to String
        factors.into_iter().map(|num: u64| {let mut string:String = glue.clone(); string.push_str(&num.to_string()); string}).collect()
    }

    pub fn nth_factorial(str_num: &str) -> String {
        let num:BigUint = num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap();

        let mut factorial_n:usize = 0;
        let mut i:usize = 0;
        let mut total:BigUint = One::one();
        while total <= num {
            i += 1;
            total = total * i;
            if total == num {
                factorial_n = i;
            }
        }

        if factorial_n != 0 {
            let mut string:String = "".to_owned();
            string.push_str("It is the ");
            string.push_str(&nth(factorial_n));
            string.push_str(" factorial number. (");
            string.push_str(&factorial_n.to_string());
            string.push_str("!)");
            string
        } else {
            "It is not a factorial number.".to_owned()
        }
    }

    pub fn is_palindrome(string: &str) -> bool {
        let half_len = string.len()/2;
        string.chars().take(half_len).eq(string.chars().rev().take(half_len))
    }

    pub fn nth_root(str_num: &str, n:usize) -> String {
        let number:BigUint = num_bigint::BigUint::from_str_radix(&str_num, 10).unwrap();
        let answer = number.nth_root(n.to_u32().unwrap()).to_owned();

        if pow(answer.to_owned(),n) == number {
            let mut string:String = "".to_owned();
            string.push_str(&answer.to_string());
            string
        } else{
            "0".to_owned()
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
            li!["Is it a prime number? (Checks numbers upto ",MAX_LEN_PRIME_CHECK.to_string()," digits in length)"],
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
                textarea![attrs!{At::Name => "number", At::Cols => "85", At::Rows => "10", At::OnKeyDown => "if (event.keyCode == 13){document.location='cruncher/'+crunchy.number.value+'/'}"}],
                br![],
                input![attrs!{At::Type => "button", At::Value => "Crunch", At::OnClick => "document.location='cruncher/'+crunchy.number.value+'/'"}],
            ],
        ],
    ]
}

fn html_factors(slug:&str, slug_len:usize, max_len_factoring:usize) -> seed::dom_types::Node<Msg> {
    if slug_len <= max_len_factoring {
        td![attrs!{At::Width => "200"},
            "It it has factors:",
            br![],
            El::from_html(&numerics_to_text::list_factors(&slug, "<br>".to_owned()))
        ]
    } else {
        td!["Number too large to factor"]
    }
}

fn html_roman(slug:&str, max_len_roman:usize) -> seed::dom_types::Node<Msg> {
    tr![
        td![attrs!{At::Width => "200"},
            "Roman Numerals:",
        ],
        td![attrs!{At::Width => "40"},
            if slug.len() <= max_len_roman {
                El::from_html(&numerics_to_text::den_to_roman(&slug))
            } else {
                El::from_html("")
            }
        ],
    ]
}

fn html_chinese(slug:&str, max_len_chinese:usize) -> seed::dom_types::Node<Msg> {
    tr![
        td![attrs!{At::Width => "200"},
            "Chinese Numerals:",
        ],
        td![style!{"vertical-align" => "middle"; "background-color" => "#FFF"},
            if slug.len() <= max_len_chinese {
                El::from_html(&numerics_to_text::den_to_chinese(&slug))
            } else {
                El::from_html("")
            }
        ]
    ]
}

fn html_egyptian(slug:&str, max_len_egyptian:usize) -> seed::dom_types::Node<Msg> {
    tr![
        td![attrs!{At::Width => "200"},
            "Egyptian Numerals:",
        ],
        td![
            if slug.len() <= max_len_egyptian {
                El::from_html(&numerics_to_text::den_to_egyptian(&slug))
            } else {
                El::from_html("")
            }
        ]
    ]
}

fn html_mersenne_prime(str_num:&str) -> seed::dom_types::Node<Msg> {
    let n = mersenne::nth_mersenne_prime(str_num) as usize;

    if n == 0 {
        span!["It is not a ",a!["mersenne prime", attrs!{At::Class => "link", At::Href => "http://en.wikipedia.org/wiki/Mersenne_prime"}],"."] 
    } else {
        span!["It is the ",nth(n)," ",a!["mersenne prime", attrs!{At::Class => "link", At::Href => "http://en.wikipedia.org/wiki/Mersenne_prime"}],"."]     
    }
}

fn nth_prime(str_num:&str, max_prime_nth_check:usize) -> usize {
    let mut answer:usize = 0;

    //if number is larger than 64bits capacity then return 0
    let check_fits_in_64bits = match str_num.parse::<usize>() {
        Ok(_v) => true,
        Err(_e) => false
    };

    if !check_fits_in_64bits {
        answer
    } else {
        let num:usize = str_num.parse().unwrap();
        let primes_list:Vec<usize> = prime::n100_prime(1,max_prime_nth_check);
        
        for n in 0..(max_prime_nth_check-1) {
            if num == primes_list[n] {
                answer = n+1;
                break;
            }
        }
        answer
    }
}

fn html_nth_prime(str_num:&str) -> seed::dom_types::Node<Msg> {
    if MAX_LEN_PRIME_CHECK < str_num.len() {
        span!["It is too large to check primality."]
    } else {
        let nth_prime_result = nth_prime(&str_num,50);
        if nth_prime_result > 0 {
            span!["It is the ",nth(nth_prime_result)," prime number."]
        } else {
            span!["It is not a prime number."]
        }
    }
}

fn html_crunched_number(slug:String) -> seed::dom_types::Node<Msg> {

    let max_len_roman = 6;
    let html_roman = html_roman(&slug, max_len_roman);

    let max_len_chinese = 6;
    let html_chinese = html_chinese(&slug, max_len_chinese);

    let max_len_egyptian = 7;
    let html_egyptian = html_egyptian(&slug, max_len_egyptian);

    let max_len_factoring = 17;   
    let html_factors = html_factors(&slug, slug.len(), max_len_factoring);

    let table_style = style!{"border" => "1px #000 solid"};

    let spoken_version_of_number:String = numerics_to_text::convert(slug.to_string());
    let title:String = match spoken_version_of_number.as_ref() {
        "Error" => slug.to_string(),//just number
        _ => vec![slug.to_string()," - ".to_string(),spoken_version_of_number].into_iter().collect()//number and text version of number e.g. 1 => one
    };

    div![style!{"width" => "75%"; "padding" => "3px"},
        br![],
        br![],
        b!["The number you submitted to be crunched was:"],
        h1![title],
        table![attrs!{At::Class => "crunchertable", At::Width => "100%"}, &table_style,
            tbody![
                tr![
                    td![
                        "It is an ",if numerics_to_text::is_odd(&slug) {"odd"} else {"even"} ," number.",
                        br![],
                        html_nth_prime(&slug),
                        br![],
                        "It is ",
                        if numerics_to_text::is_palindrome(&slug) { "" } else { "not " },
                        "palindromic.",
                        br![],
                        html_mersenne_prime(&slug),
                        br![],
                        //TODO hardcoded example value
                        //"It is not a ",a!["fermat prime", attrs!{At::Class => "link", At::Href => "https://www.fermatsearch.org/"}],".",
                        //br![],
                        //TODO hardcoded example value
                        //"It is not a ",a!["perfect number", attrs!{At::Class => "link", At::Href => "https://en.wikipedia.org/wiki/Perfect_number"}],".",
                        //br![],
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "crunchertable", At::Width => "100%"}, &table_style,
            tbody![
                tr![
                    td![
                        //TODO hardcoded example value
                        //"It is not a triangle number.",
                        //br![],
                        if numerics_to_text::nth_root(&slug, 2) != "0" { format!("It is the {} square number.",&nth(numerics_to_text::nth_root(&slug, 2).parse::<usize>().unwrap())) } else { "It is not a square number.".to_owned() },
                        br![],
                        if numerics_to_text::nth_root(&slug, 3) != "0" { format!("It is the {} cube number.",&nth(numerics_to_text::nth_root(&slug, 3).parse::<usize>().unwrap())) } else { "It is not a cube number.".to_owned() },
                        br![],
                        br![],
                        numerics_to_text::nth_factorial(&slug),
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "crunchertable", At::Width => "100%"}, &table_style,
            tbody![
                tr![
                    html_factors,
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "crunchertable", At::Width => "100%"}, &table_style,
            tbody![
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 2 (Binary):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 2),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 3 (Ternary):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 3),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 4 (Quaternary):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 4),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 5 (Quinary):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 5),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 8 (Octal):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 8),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 10 (Denary):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 10),
                    ],
                ],
                tr![
                    td![attrs!{At::Width => "200"},
                        "Base 16 (Hexadecimal):",
                    ],
                    td![
                        numerics_to_text::dec_to_base(&slug, 16).to_uppercase(),
                    ],
                ],
            ],
        ],
        br![],
        br![],
        table![attrs!{At::Class => "crunchertable", At::Width => "100%"}, &table_style,
            tbody![
                html_roman,
                html_egyptian,
                html_chinese,
                tr![
                    td![attrs!{At::Width => "200"},
                        "Babylonian Numerals:",
                    ],
                    td![style!{"vertical-align" => "middle"; "background-color" => "#FFF"},
                        El::from_html(&numerics_to_text::den_to_babylonian(&slug)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numerics_to_text_convert_test() {
        assert_eq!(numerics_to_text::convert("170".to_string()), "one hundred seventy");
        assert_eq!(numerics_to_text::convert("90001".to_string()), "ninety thousand one");
        assert_eq!(numerics_to_text::convert("1001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001000".to_string()), "one noventrigintillion one octotrigintillion one septentrigintillion one sestrigintillion one quinquatrigintillion one quattuortrigintillion one trestrigintillion one duotrigintillion one untrigintillion one trigintillion one novemvigintillion one octovigintillion one septemvigintillion one sesvigintillion one quinquavigintillion one quattuorvigintillion one tresvigintillion one duovigintillion one unvigintillion one vigintillion one novemdecillion one octodecillion one septendecillion one sexdecillion one quindecillion one quattuordecillion one tredecillion one duodecillion one undecillion one decillion one nonillion one octillion one septillion one sextillion one quintillion one quadrillion one trillion one billion one million one thousand");
        // test anything over triplets vector size * 3 returns "Error", and if you want to extend this limit, then more definitions should be added to triplets within convert()
        assert_eq!(numerics_to_text::convert("1999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999".to_string()), "Error");
    }

    #[test]
    fn numerics_to_text_is_odd_test() {
        assert_eq!(numerics_to_text::is_odd("170"), false);
        assert_eq!(numerics_to_text::is_odd("90001"), true);
        assert_eq!(numerics_to_text::is_odd("1001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001000"), false);
        assert_eq!(numerics_to_text::is_odd("1001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001001"), true);
    }

    #[test]
    fn dec_to_base_test() {
        assert_eq!(numerics_to_text::dec_to_base("16",16), "10");
        assert_eq!(numerics_to_text::dec_to_base("9003",5), "242003");
        assert_eq!(numerics_to_text::dec_to_base("2147483648",16), "80000000");//test for 32bit overflow
        assert_eq!(numerics_to_text::dec_to_base("18446744073709551616",16), "10000000000000000");//test for 64bit overflow
    }

    #[test]
    fn den_to_roman_test() {
        assert_eq!(numerics_to_text::den_to_roman("5"), "V");
        assert_eq!(numerics_to_text::den_to_roman("10"), "X");
        assert_eq!(numerics_to_text::den_to_roman("12"), "XII");
        assert_eq!(numerics_to_text::den_to_roman("57"), "LVII");
        assert_eq!(numerics_to_text::den_to_roman("2002"), "MMII");
    }

    #[test]
    fn den_to_egyptian_test() {
        //TODO: egyptian tests
    }

    #[test]
    fn den_to_babylonian_test() {
        assert_eq!(numerics_to_text::den_to_babylonian("9003")," &nbsp; <img height=\"15\" src=\"https://static.bigprimes.net/imgs/babnumbers/bab_2.gif\" alt=\"2\"> &nbsp; <img height=\"15\" src=\"https://static.bigprimes.net/imgs/babnumbers/bab_30.gif\" alt=\"30\"> &nbsp; <img height=\"15\" src=\"https://static.bigprimes.net/imgs/babnumbers/bab_3.gif\" alt=\"3\">");
    }

    #[test]
    fn den_to_chinese_test() {
        assert_eq!(numerics_to_text::den_to_chinese("20"), "&#36019;&#25342;");
    }

    #[test]
    fn list_factors_test() {
        //todo: this test shows we have a comma prefix, tidier if that didn't happen
        assert_eq!(numerics_to_text::list_factors("20",",".to_owned()), ",1,2,4,5,10,20");
    }

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime("2",10), 1);
        assert_eq!(nth_prime("3",10), 2);
        assert_eq!(nth_prime("4",10), 0);
        assert_eq!(nth_prime("5",10), 3);
        assert_eq!(nth_prime("5555555555555555555555555555555555555555555555555555555",10), 0);
    }
}

//todo: this file is vast, break it up!
