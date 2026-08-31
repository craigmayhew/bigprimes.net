//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use big_primes::pages::primalitytest::{check, go_crunch, go_list, listy};
use seed::prelude::{js_sys, wasm_bindgen::closure::Closure, web_sys, JsCast, JsValue};
use seed::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("browser window should exist")
        .document()
        .expect("browser document should exist")
}

fn attach_fixture(id: &str, html: &str) -> web_sys::Element {
    let document = document();
    let fixture = document
        .create_element("div")
        .expect("test fixture should be created");
    fixture.set_id(id);
    fixture.set_inner_html(html);
    document
        .body()
        .expect("browser document should have a body")
        .append_child(&fixture)
        .expect("test fixture should be attached");
    fixture
}

fn current_relative_url() -> String {
    let location = web_sys::window()
        .expect("browser window should exist")
        .location();
    format!(
        "{}{}{}",
        location.pathname().expect("pathname should be available"),
        location.search().expect("search should be available"),
        location.hash().expect("hash should be available")
    )
}

fn set_relative_url(url: &str) {
    web_sys::window()
        .expect("browser window should exist")
        .history()
        .expect("browser history should exist")
        .replace_state_with_url(&JsValue::NULL, "", Some(url))
        .expect("test URL should be replaceable");
}

fn mount_route(path: &str, fixture_id: &str) -> web_sys::Element {
    set_relative_url(path);
    let fixture = attach_fixture(fixture_id, "");
    big_primes::start_at(fixture_id);
    fixture
}

async fn next_animation_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let resolve = resolve.clone();
        let callback = Closure::once(move |_: f64| {
            resolve
                .call0(&JsValue::NULL)
                .expect("animation frame promise should resolve");
        });
        web_sys::window()
            .expect("browser window should exist")
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .expect("animation frame should be requested");
        callback.forget();
    });

    JsFuture::from(promise)
        .await
        .expect("animation frame should complete");
}

fn dispatch_mouseup_on_last_button(fixture: &web_sys::Element, selector: &str) {
    let buttons = fixture
        .query_selector_all(selector)
        .expect("generate buttons should be queryable");
    assert!(
        buttons.length() > 0,
        "at least one generate button should exist"
    );
    buttons
        .item(buttons.length() - 1)
        .expect("last generate button should exist")
        .dispatch_event(
            &web_sys::MouseEvent::new("mouseup").expect("mouseup event should be created"),
        )
        .expect("mouseup event should be dispatched");
}

fn assert_download_link(fixture: &web_sys::Element, expected_href: &str) {
    let link = fixture
        .query_selector(r#"a[download="P1.txt"]"#)
        .expect("download link should be queryable")
        .expect("download link should be rendered");
    assert_eq!(link.get_attribute("download").as_deref(), Some("P1.txt"));
    assert_eq!(link.get_attribute("href").as_deref(), Some(expected_href));
}

#[wasm_bindgen_test]
fn go_crunch_reads_hexadecimal_input_and_updates_the_page() {
    let fixture = attach_fixture(
        "go-crunch-fixture",
        r#"<input id="number" value="0x1f"><div id="prime_check_output"></div>"#,
    );

    go_crunch();

    let output = document()
        .get_element_by_id("prime_check_output")
        .expect("output element should exist");
    assert_eq!(output.inner_html(), "31 is a (proven) prime!");

    fixture.remove();
}

#[wasm_bindgen_test]
fn go_list_updates_the_page_for_valid_input() {
    let fixture = attach_fixture(
        "go-list-valid-fixture",
        concat!(
            r#"<input id="start" value="14">"#,
            r#"<input id="primes" value="3">"#,
            r#"<textarea id="prime_check_list_output"></textarea>"#
        ),
    );

    go_list();

    let output = document()
        .get_element_by_id("prime_check_list_output")
        .expect("prime list output should exist");
    assert_eq!(
        output.inner_html(),
        concat!(
            "17 is a (proven) prime!\n",
            "19 is a (proven) prime!\n",
            "23 is a (proven) prime!\n"
        )
    );

    fixture.remove();
}

#[wasm_bindgen_test]
fn go_list_displays_validation_message_for_invalid_input() {
    let fixture = attach_fixture(
        "go-list-invalid-fixture",
        concat!(
            r#"<input id="start" value="not-a-number">"#,
            r#"<input id="primes" value="3">"#,
            r#"<textarea id="prime_check_list_output"></textarea>"#
        ),
    );

    go_list();

    let output = document()
        .get_element_by_id("prime_check_list_output")
        .expect("prime list output should exist");
    assert_eq!(
        output.inner_html(),
        "Please enter valid unsigned integers for the start and count."
    );

    fixture.remove();
}

#[wasm_bindgen_test]
fn application_mounts_and_renders_representative_routes() {
    let original_url = current_relative_url();
    let routes = [
        ("/", "home-route-fixture", "News"),
        (
            "/archive/prime/",
            "prime-route-fixture",
            "The Prime Numbers",
        ),
        ("/cruncher/31/", "cruncher-route-fixture", "31 - thirty one"),
        ("/not-a-real-page/", "error-route-fixture", "404"),
    ];

    for (path, fixture_id, expected_heading) in routes {
        let fixture = mount_route(path, fixture_id);
        let heading = fixture
            .query_selector("h1")
            .expect("heading should be queryable")
            .expect("route should render a heading");
        assert_eq!(heading.text_content().as_deref(), Some(expected_heading));
        fixture.remove();
    }

    set_relative_url(&original_url);
}

#[wasm_bindgen_test(async)]
async fn mounted_number_cruncher_form_uses_seed_event_handlers() {
    let original_url = current_relative_url();
    let fixture = mount_route("/cruncher/", "number-cruncher-controls-fixture");
    let input = fixture
        .query_selector("#cruncher-form textarea")
        .expect("number cruncher input should be queryable")
        .expect("number cruncher input should be rendered")
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .expect("number cruncher input should be an HTML textarea element");
    input.set_value("31");
    input
        .dispatch_event(&web_sys::Event::new("input").expect("input event should be created"))
        .expect("input event should be dispatched");
    fixture
        .query_selector("#cruncher-form input[type=submit]")
        .expect("crunch button should be queryable")
        .expect("crunch button should be rendered")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("crunch button should be an HTML element")
        .click();

    next_animation_frame().await;

    let heading = fixture
        .query_selector("h1")
        .expect("heading should be queryable")
        .expect("number cruncher result should render a heading");
    assert_eq!(heading.text_content().as_deref(), Some("31 - thirty one"));

    fixture.remove();
    set_relative_url(&original_url);
}

#[wasm_bindgen_test(async)]
async fn mounted_primality_controls_handle_input_and_click_events() {
    let original_url = current_relative_url();
    let fixture = mount_route("/primalitytest/", "primality-controls-fixture");
    let input = fixture
        .query_selector("#number")
        .expect("number input should be queryable")
        .expect("number input should be rendered")
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("number input should be an HTML input element");
    input.set_value("0x1f");
    input
        .dispatch_event(&web_sys::Event::new("input").expect("input event should be created"))
        .expect("input event should be dispatched");
    fixture
        .query_selector("#primetest button")
        .expect("check button should be queryable")
        .expect("check button should be rendered")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("check button should be an HTML element")
        .click();

    next_animation_frame().await;

    let output = fixture
        .query_selector("#prime_check_output")
        .expect("primality output should be queryable")
        .expect("primality output should be rendered");
    assert_eq!(output.inner_html(), "31 is a (proven) prime!");

    fixture.remove();
    set_relative_url(&original_url);
}

#[wasm_bindgen_test(async)]
async fn mersenne_download_button_renders_filename_and_data_url() {
    let original_url = current_relative_url();
    let fixture = mount_route("/archive/mersenne/", "mersenne-download-fixture");

    dispatch_mouseup_on_last_button(&fixture, ".mersennetable button");
    next_animation_frame().await;

    assert_download_link(&fixture, "data:text/plain,3");

    fixture.remove();
    set_relative_url(&original_url);
}

#[wasm_bindgen_test(async)]
async fn perfect_download_button_renders_filename_and_data_url() {
    let original_url = current_relative_url();
    let fixture = mount_route("/archive/perfect/", "perfect-download-fixture");

    dispatch_mouseup_on_last_button(&fixture, ".perfecttable button");
    next_animation_frame().await;

    assert_download_link(&fixture, "data:text/plain,6");

    fixture.remove();
    set_relative_url(&original_url);
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
