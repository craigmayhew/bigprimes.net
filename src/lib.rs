#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use seed::prelude::*;

pub mod pages;
pub mod utils;

#[derive(Copy, Clone)]
pub enum Page {
    ContactUs,
    Downloads,
    Faq,
    Home,
    Status,
    FermatArchive,
    FibonacciArchive,
    MersenneArchive,
    PerfectArchive,
    PrimalityChecker,
    PrimeNumbersArchive,
    NumberCruncher,
}

// Model
struct Model {
    page: Page,
    slug: std::string::String,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Home,
            slug: "".to_owned(),
        }
    }
}

// Update
#[derive(Clone)]
pub enum Msg {
    ChangePage(Page, std::string::String),
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(page, slug) => {model.page = page; model.slug = slug}
    }
}

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output has to implement trait `ElContainer`.
fn view(model: &Model) -> impl View<Msg> {
    match model.page {
        Page::ContactUs => pages::contact::render(),
        Page::Downloads => pages::downloads::render(),
        Page::Faq => pages::faq::render(),
        Page::FermatArchive => pages::archive::fermat::render(),
        Page::FibonacciArchive => pages::archive::fibonacci::render(model.slug.to_owned()),
        Page::Home => pages::home::render(),
        Page::MersenneArchive => pages::archive::mersenne::render(),
        Page::NumberCruncher => pages::cruncher::render(model.slug.to_owned()),
        Page::PerfectArchive => pages::archive::perfect::render(),
        Page::PrimalityChecker => pages::primalitytest::render(),
        Page::PrimeNumbersArchive => pages::archive::prime::render(model.slug.to_owned()),
        Page::Status => pages::status::render(),
    }
}

impl ToString for Page {
    fn to_string(&self) -> String {
        // Eg for url routing
        match self {
            Page::ContactUs => "contactus".into(),
            Page::Downloads => "downloads".into(),
            Page::Faq => "faq".into(),
            Page::FermatArchive => "fermat".into(),
            Page::FibonacciArchive => "fibonacci".into(),//TODO: check this is right, might need the archive/ prefix!
            Page::Home => "home".into(),
            Page::Status => "status".into(),
            Page::MersenneArchive => "mersenne".into(),
            Page::NumberCruncher => "cruncher".into(),
            Page::PerfectArchive => "perfect".into(),
            Page::PrimalityChecker => "primalitytest".into(),
            Page::PrimeNumbersArchive => "primes".into(),//TODO: check this is right, might need the archive/ prefix!
        }
    }
}

fn routes(url: seed::Url) -> Msg {

    let empty_string = "".to_owned();

    if url.path.is_empty() {
        return Msg::ChangePage(Page::Home, empty_string)
    }

    match url.path[0].as_ref() {
        "archive" => {
            // Determine if we are at the archive page, or a subpage
            match url.path[1].as_ref() {
                "fermat" => Msg::ChangePage(Page::FermatArchive, empty_string),
                "fibonacci" => {
                    match url.path.get(2).as_ref() {
                        Some(_slug) => Msg::ChangePage(Page::FibonacciArchive, url.path[2].to_owned()),
                        None => Msg::ChangePage(Page::FibonacciArchive, "1".to_owned()),
                    }
                },
                "mersenne" => Msg::ChangePage(Page::MersenneArchive, empty_string),
                "perfect" => Msg::ChangePage(Page::PerfectArchive, empty_string),
                "prime" => {
                    match url.path.get(2).as_ref() {
                        Some(_slug) => Msg::ChangePage(Page::PrimeNumbersArchive, url.path[2].to_owned()),
                        None => Msg::ChangePage(Page::PrimeNumbersArchive, "1".to_owned()),
                    }
                },
                _ => Msg::ChangePage(Page::Home, empty_string)//TODO: add archive page
            }
        },
        "contactus" => Msg::ChangePage(Page::ContactUs, empty_string),
        "cruncher" => {
            match url.path.get(1).as_ref() {
                Some(_slug) => Msg::ChangePage(Page::NumberCruncher, url.path[1].to_owned()),
                None => Msg::ChangePage(Page::NumberCruncher, empty_string),
            }
        },
        "downloads" => Msg::ChangePage(Page::Downloads, empty_string),
        "faq" => Msg::ChangePage(Page::Faq, empty_string),
        "primalitytest" => Msg::ChangePage(Page::PrimalityChecker, empty_string),
        "status" => Msg::ChangePage(Page::Status, empty_string),
        _ => Msg::ChangePage(Page::Home, empty_string)
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view)
        .mount("app")
        .routes(routes)
        //.window_events(window_events)
        .finish()
        .run();
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut model = Model::default();
    update(routes(url), &mut model, orders);
    model
}