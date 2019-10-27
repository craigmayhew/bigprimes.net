use seed::prelude::*;
use crate::Msg;

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigUint};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn nth(num: usize) -> String {
    //format a number to say 1st, 2nd, 3rd etc
    format!("{}{}", num, match (num % 10, num % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    })
}

pub fn save_as_file(filename:String, filecontent:String) -> seed::dom_types::Node<Msg> {
        let href:String = format!("data:text/plain,{}",&filecontent);
        a![attrs!{At::Download => &filename, At::Href => &href}, "TXT"]
}

pub fn generate_file(n:u64, value:BigUint) -> seed::dom_types::Node<Msg> {
    let download_filename:String = format!("P{}.txt",&n.to_string());
    save_as_file(download_filename, value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn save_as_file_test() {
        let test_file = save_as_file(
            String::from("example name"),
            String::from("some content")
        );

        assert_eq!(test_file.get_text(), "TXT");
        //todo: we are only testing the link text currently
    }
}
