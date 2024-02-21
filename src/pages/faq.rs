use crate::Msg;
use seed::prelude::*;

pub fn render() -> Node<Msg> {
    div![
        h1!["FAQ"],
        br![],
        br![],
        br![],
        b!["Is x prime?"],
        p!["Please try our number cruncher. If your question is more complex then try using the contact form. I can't promise a quick response though."],
        b!["I've found a bug/mistake on bigprimes.net!"],
        p!["Please opening an issue on ",a!["github", attrs!{At::Class => "link", At::Href => "https://github.com/craigmayhew/bigprimes.net/"}]],
        b!["What is the first set of 100 numbers (e.g. 100-199, 1300-1399, 312300-312399) that does not contain a prime number?"],
        p!["The first occurrence is between the prime numbers 1671781 and 1671907 on ",a!["this", attrs!{At::Class => "link", At::Href => "/archive/prime/1262/"}]," page (near bottom of 3rd column)."],
        b!["What is the smallest prime number that contains all digits 0 to 9 in sequence?"],
        p!["100123456789"],
        b!["What percentage of prime numbers end in 1,3,7 and 9?"],
        p!["25% of prime numbers end in a 1",br![],"25% of prime numbers end in a 3",br![],"25% of prime numbers end in a 7",br![],"25% of prime numbers end in a 9"],        
    ]
}
