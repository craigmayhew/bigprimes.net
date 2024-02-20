use crate::Msg;
use seed::prelude::*;

#[macro_export]
macro_rules! bp_table_row {
    ($($date:expr, $description:expr),*) => {
        tr![td![$($date),*],td![El::from_html(None, $($description),*)]]
    };
}

pub fn render() -> Node<Msg> {
    div![
        h1!["News"],
        br![],
        br![],
        br![],
        table![
            attrs!{At::Class => "text"},
            tbody![
                tr![
                    //TODO: change this to styles
                    td![attrs!{At::Width => "200"},"Date"],
                    td![attrs!{At::Width => "500"},"News"],
                ],
                bp_table_row!("2nd April 2022","bigprimes.net web2 version is now deployed to Cloudflare Pages and no longer AWS CloudFront."),
                bp_table_row!("26th June 2021","bigprimes.net is now using the <a class=\"link\" href=\"https://github.com/thedodd/trunk\">trunk</a> WASM web application bundler for Rust."),
                bp_table_row!("28th July 2019","bigprimes.net has been rewritten from the ground up using rust!<br>All archives have been retired in favour of client side processing in web assembly."),
                bp_table_row!("28th July 2019","Shout out to Gavin, who took the time to memorise all the news entries on the website!"),
                bp_table_row!("22nd January 2019","<a class=\"link\" href=\"https://github.com/craigmayhew/ethereum-contracts\">Ethereum bounty contracts</a> have now been added for anyone that would like to try to push the boundaries of human knowledge. The first contract is a competition for the correct solution to a³+b³+c³ = 33. Tweet me if you need assistance using these contracts!"),
                bp_table_row!("10th March 2018","Added torrents for the <del>first million primes</del> and <del>first billion prime numbers</del>. (July 2021 update, these torrents have been retired)"),
                bp_table_row!("12th November 2017","Site is now running on AWS Lambda, but can still fallback to regular VM hosting if ever needed. Site is now ~40% covered by unit tests."),
                bp_table_row!("13th November 2016","Site source code has been converted to PHP7 with Silex framework. Some methods have also been moved to cpp from PHP <a class=\"link\" href=\"https://github.com/craigmayhew/bigprimes.net/\">https://github.com/craigmayhew/bigprimes.net/</a>"),
                bp_table_row!("28th February 2014","You can now find the websites php source code here <a class=\"link\" href=\"https://github.com/craigmayhew/bigprimes.net/\">https://github.com/craigmayhew/bigprimes.net/</a>"),
                bp_table_row!("19th November 2011","Thank you to Yu Kwong Yiu Wilson for the donation!"),
                bp_table_row!("13th July 2011","We are retiring the distributed computing client. Thank you all for your help and processing cycles. Watch this space for an open source GPU client coming soon..."),
                bp_table_row!("10th February 2010","We have added a page that shows the sum of the digits of prime numbers (now retired)."),
                bp_table_row!("1st February 2010","Thanks to everyone for donating computer time, we are now upto 1.4 billion primes. We now also have a facebook group where you get news updates and show your support for the project!"),
                bp_table_row!("28th September 2009","Thanks for all your help! We made it to one billion prime numbers! Please keep on donating processor time as we still have plenty of storage for the prime numbers. Our database is now more than double the size (<a class=\"link\" href=\"/archive/prime/3000000/\">300 million primes</a>) and we've put in the capacity to keep growing. To view our current progress please visit the <del>status</del> page."),
                bp_table_row!("18th August 2009","It's been a while but we have been very busy. We've put together a distributed computing client written in javascript. Our database is now more than double the size (<a class=\"link\" href=\"/archive/prime/3000000/\">300 million primes</a>) and we've put in the capacity to keep growing. To view our current progress please visit the <del>status</del> page."),
                bp_table_row!("17th July 2008","We've now added a Forum (now retired) which can be found on the navigation links on the left."),
                bp_table_row!("14th July 2008","Someone named Kirk just emailed me to say the download links wern't working. I've now fixed this. Thanks Kirk!"),
                bp_table_row!("4th July 2008","Added much faster code to the cruncher so that we can handle more of your page loads! Lots of little updates and some reorganizing of the site."),
                bp_table_row!("23rd June 2007","Added the <a class=\"link\" href=\"/downloads/\">Downloads</a> section."),
                bp_table_row!("11th February 2007","Added the Fibonacci number archive containing the first 70331 Fibonacci numbers."),
                bp_table_row!("16th January 2007","Added the 44th Mersenne Prime number to the archive."),
                bp_table_row!("12th January 2007","The Archives are now more organized."),
                bp_table_row!("28th September 2005","We have managed to archive the first 100 million prime numbers."),
                bp_table_row!("20th May 2005","We have managed to archive the first 75 million prime numbers."),
                bp_table_row!("16th May 2005","We have managed to archive the first 50 million prime numbers."),
                bp_table_row!("14th May 2005","We have managed to archive the first 10 million prime numbers."),
                bp_table_row!("13th May 2005","We have managed to archive the first 1 million prime numbers."),
            ],
        ],
    ]
}
