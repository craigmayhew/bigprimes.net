use seed::prelude::*;
use crate::Msg;

fn nth(num: usize) -> String {
    format!("{}{}", num, match (num % 10, num % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    })
}

/*
 * Return a vector of fibonnaci numbers from n to n+count
 */
fn nth_fibonacci(n: usize, count: usize) -> Vec<usize> {
    let count = count - 1;
    let mut x = vec![1, 1];
    for i in 2..(n+count) {
        let next_x = x[i - 1] + x[i - 2];
        x.push(next_x)
    }
    x[(n-1)..(n+count)].to_vec()
}

pub fn render(slug:String) -> seed::dom_types::Node<Msg> {
    let numbers_per_page:usize = 25;
    //TODO: what occurs if slug_int is an overflow? i.e. greater tha 64bits
    let slug_int:usize = slug.parse().unwrap();
    let fib_vec = nth_fibonacci(slug_int, numbers_per_page);

    let mut fib_vec_formatted = vec![];
    for i in 0..numbers_per_page {
        fib_vec_formatted.push(span![fib_vec[i].to_string()]);
        fib_vec_formatted.push(br![]);
    }

    div![
        h1!["The Fibonacci Numbers"],
        br![],
        "This page shows the ",nth(slug_int)," fibonacci number followed by the next ",numbers_per_page.to_string(),".",
        br![],
        br![],
        fib_vec_formatted,
        br![],
        a!["previous ",numbers_per_page.to_string()," fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],
        br![],
        a!["next ",numbers_per_page.to_string()," fibonacci numbers", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/70332/"}],
        br![],
        br![],
        a!["100th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/100/"}],
        br![],
        a!["1000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/1000/"}],
        br![],
        a!["10000th Fibonacci Number", attrs!{At::Class => "link", At::Href => "/archive/fibonacci/10000/"}],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_fibonacci_test() {
        assert_eq!(nth_fibonacci(1,2), vec![1,1]);
        assert_eq!(nth_fibonacci(1,8), vec![1,1,2,3,5,8,13,21]);
        assert_eq!(nth_fibonacci(3,9), vec![2,3,5,8,13,21,34,55,89]);
    }

    #[test]
    fn nth_test() {
        assert_eq!(nth(7), "7th");
        assert_ne!(nth(7), "1st");
        assert_eq!(nth(7000000000000000000), "7000000000000000000th");
        assert_eq!(nth(7000000000000000001), "7000000000000000001st");
        assert_eq!(nth(7000000000000000002), "7000000000000000002nd");
        assert_eq!(nth(7000000000000000003), "7000000000000000003rd");
        assert_eq!(nth(7000000000000000011), "7000000000000000011th");
        assert_eq!(nth(7000000000000000012), "7000000000000000012th");
        assert_eq!(nth(7000000000000000013), "7000000000000000013th");
    }
}