use crate::Msg;
use chrono::{NaiveDate};
use seed::prelude::*;

pub fn render() -> seed::dom_types::Node<Msg> {
    div![
        h1!["News"],
        br![],
        br![],
        br![],
        El::from_html("
        <table cellpadding=\"3\" cellspacing=\"0\" border=\"1\" class=\"text\" style=\"width:80%;\">
        <tbody>
        <tr><td width=\"100\">Date</td><td width=\"300\">News</td></tr>
        <tr><td>28th July 2019</td><td>bigprimes.net has been rewritten from the ground up using rust!</td></tr>
        <tr><td>22nd January 2019</td><td><a class=\"link\" href=\"https://github.com/craigmayhew/ethereum-contracts\">Ethereum bounty contracts</a> have now been added for anyone that would like to try to push the boundaries of human knowledge. The first contract is a competition for the correct solution to a³+b³+c³ = 33. Tweet me if you need assistance using these contracts!</td></tr>
        <tr><td>10th March 2018</td><td>Added torrents for the <a class=\"link\" href=\"//static.bigprimes.net/archive/one-million-primes.7z.torrent\">first million primes</a> and <a class=\"link\" href=\"//static.bigprimes.net/archive/one-billion-primes.7z.torrent\">first billion prime numbers</a>.</td></tr>
        <tr><td>12th November 2017</td><td>Site is now running on AWS Lambda, but can still fallback to regular VM hosting if ever needed. Site is now ~40% covered by unit tests.</td></tr>
        <tr><td>13th November 2016</td><td>Site source code has been converted to PHP7 with Silex framework. Some methods have also been moved to cpp from PHP <a class=\"link\" href=\"https://github.com/craigmayhew/bigprimes.net/\">https://github.com/craigmayhew/bigprimes.net/</a></td></tr>
        <tr><td>28th February 2014</td><td>You can now find the websites php source code here <a class=\"link\" href=\"https://github.com/craigmayhew/bigprimes.net/\">https://github.com/craigmayhew/bigprimes.net/</a></td></tr>
        <tr><td>19th November 2011</td><td>Thank you to Yu Kwong Yiu Wilson for the donation!</td></tr>
        <tr><td>13th July 2011</td><td>We are retiring the distributed computing client. Thank you all for your help and processing cycles.
        Watch this space for an open source GPU client coming soon...</td></tr>
        <tr><td>10th February 2010</td><td>We have added a page that shows the sum of the digits of prime numbers (now retired).</td></tr>
        <tr><td>1st February 2010</td><td>Thanks to everyone for donating computer time, we are now upto 1.4 billion primes. We now also have a facebook group where you get news updates and show your support for the project!</td></tr>
        <tr><td>28th September 2009</td><td>Thanks for all your help! We made it to one billion prime numbers! Please keep on donating processor time as we still have plenty of storage for the prime numbers. Our database is now more than double the size (<a class=\"link\" href=\"/archive/prime/3000000/\">300 million primes</a>) and we've put in the capacity to keep growing. To view our current progress please visit the <a class=\"link\" href=\"/status/\">status</a> page.</td></tr>
        <tr><td>18th August 2009</td><td>It's been a while but we have been very busy. We've put together a distributed computing client written in javascript. Our database is now more than double the size (<a class=\"link\" href=\"/archive/prime/3000000/\">300 million primes</a>) and we've put in the capacity to keep growing. To view our current progress please visit the <a class=\"link\" href=\"/status/\">status</a> page.</td></tr>
        <tr><td>17th July 2008</td><td>We've now added a Forum (now retired) which can be found on the navigation links on the left.</td></tr>
        <tr><td>14th July 2008</td><td>Someone named Kirk just emailed me to say the download links wern't working. I've now fixed this. Thanks Kirk!</td></tr>
        <tr><td>4th July 2008</td><td>Added much faster code to the cruncher so that we can handle more of your page loads! Lots of little updates and some reorganizing of the site.</td></tr>
        <tr><td>23rd June 2007</td><td>Added the <a class=\"link\" href=\"/downloads/\">Downloads</a> section.</td></tr>
        <tr><td>11th February 2007</td><td>Added the Fibonacci number archive containing the first 70331 Fibonacci numbers.</td></tr>
        <tr><td>16th January 2007</td><td>Added the 44th Mersenne Prime number to the archive.</td></tr>
        <tr><td>12th January 2007</td><td>The Archives are now more organized.</td></tr>
        <tr><td>28th September 2005</td><td>We have managed to archive the first 100 million prime numbers.</td></tr>
        <tr><td>20th May 2005</td><td>We have managed to archive the first 75 million prime numbers.</td></tr>
        <tr><td>16th May 2005</td><td>We have managed to archive the first 50 million prime numbers.</td></tr>
        <tr><td>14th May 2005</td><td>We have managed to archive the first 10 million prime numbers.</td></tr>
        <tr><td>13th May 2005</td><td>We have managed to archive the first 1 million prime numbers.</td></tr>
        </tbody>
        </table>"),
    ]
}