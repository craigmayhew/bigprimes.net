//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use big_primes::pages::primalitytest::{check, go_crunch, listy};
use seed::prelude::web_sys;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn go_crunch_reads_hexadecimal_input_and_updates_the_page() {
    let document = web_sys::window()
        .expect("browser window should exist")
        .document()
        .expect("browser document should exist");
    let fixture = document
        .create_element("div")
        .expect("test fixture should be created");
    fixture
        .set_inner_html(r#"<input id="number" value="0x1f"><div id="prime_check_output"></div>"#);
    document
        .body()
        .expect("browser document should have a body")
        .append_child(&fixture)
        .expect("test fixture should be attached");

    go_crunch();

    let output = document
        .get_element_by_id("prime_check_output")
        .expect("output element should exist");
    assert_eq!(output.inner_html(), "31 is a (proven) prime!");

    fixture.remove();
}

#[wasm_bindgen_test]
fn primality_checker_supports_the_full_u64_range_in_wasm() {
    let prime = check("18446744073709551557".to_owned());
    assert!(prime.is_prime);
    assert_eq!(prime.result, "18446744073709551557 is a (proven) prime.");

    let composite = check("18446744073709551615".to_owned());
    assert!(!composite.is_prime);
    assert_eq!(
        composite.result,
        "18446744073709551615 is not a prime! It is 3 * 6148914691236517205"
    );
}

#[wasm_bindgen_test]
fn prime_list_contains_the_requested_number_of_primes() {
    assert_eq!(
        listy(14, 3),
        concat!(
            "17 is a (proven) prime!\n",
            "19 is a (proven) prime!\n",
            "23 is a (proven) prime!\n"
        )
    );
}
