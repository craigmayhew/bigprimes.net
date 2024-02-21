#![feature(test)]
#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use seed::prelude::*;

pub mod pages;
pub mod utils;

#[derive(Copy, Clone)]
pub enum Page {
    Archive,
    Contact,
    Error,
    Faq,
    Home,
    FermatArchive,
    FibonacciArchive,
    MersenneArchive,
    PerfectArchive,
    PrimalityChecker,
    PrimeNumbersArchive,
    NumberCruncher,
}

// Model
pub struct Model {
    download: pages::archive::perfect::perfects_utils::PerfectDownload,
    primalitycheckerfieldvalues: pages::primalitytest::PrimalityTestPageInputs,
    page: Page,
    slug: std::string::String,
}

impl Model {
    fn new(url: Url) -> Self {
        let (page, slug) = routes(url.clone());

        Self {
            download: pages::archive::perfect::perfects_utils::PerfectDownload { n: 0, p: 0 },
            primalitycheckerfieldvalues: pages::primalitytest::PrimalityTestPageInputs {
                number: 31,
                primes: 5,
                start: 65000,
            },
            page: page,
            slug: slug,
        }
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model::new(url)
}

// Update
#[derive(Clone)]
pub enum Msg {
    GenerateMersenneDownload(
        web_sys::MouseEvent,
        pages::archive::mersenne::mersenne_utils::MersenneDownload,
    ),
    GeneratePerfectDownload(
        web_sys::MouseEvent,
        pages::archive::perfect::perfects_utils::PerfectDownload,
    ),
    UrlChanged(subs::UrlChanged),
    PrimalityChecker(()),
    PrimalityCheckerInputNumberValueChanged(String),
    PrimalityCheckerInputPrimesValueChanged(String),
    PrimalityCheckerInputStartValueChanged(String),
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        //TODO: IS there a bug here that model.download.n = perfect_download is used for mersenne?
        Msg::GenerateMersenneDownload(_event, perfect_download) => {
            model.download.n = perfect_download.n;
            model.download.p = perfect_download.p
        }
        Msg::GeneratePerfectDownload(_event, perfect_download) => {
            model.download.n = perfect_download.n;
            model.download.p = perfect_download.p
        }
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let (page, slug) = routes(url);
            model.download =
                pages::archive::perfect::perfects_utils::PerfectDownload { n: 0, p: 0 };
            model.page = page;
            model.slug = slug;
            ()
        }
        //todo: split this into two, one for ok button and have the function calls in here rather than in the view
        Msg::PrimalityChecker(_) => {}
        Msg::PrimalityCheckerInputNumberValueChanged(value) => {
            model.primalitycheckerfieldvalues.number = value.parse::<u64>().unwrap();
        }
        Msg::PrimalityCheckerInputPrimesValueChanged(value) => {
            model.primalitycheckerfieldvalues.primes = value.parse::<u64>().unwrap();
        }
        Msg::PrimalityCheckerInputStartValueChanged(value) => {
            model.primalitycheckerfieldvalues.start = value.parse::<u64>().unwrap();
        }
    }
}

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output has to implement trait `ElContainer`.
fn view(model: &Model) -> Node<Msg> {
    match model.page {
        Page::Archive => pages::archive::index::render(),
        Page::Contact => pages::contact::render(),
        Page::Error => pages::error::render(),
        Page::Faq => pages::faq::render(),
        Page::FermatArchive => pages::archive::fermat::render(),
        Page::FibonacciArchive => pages::archive::fibonacci::render(model.slug.to_owned()),
        Page::Home => pages::home::render(),
        Page::MersenneArchive => pages::archive::mersenne::render(&model),
        Page::NumberCruncher => pages::cruncher::render(model.slug.to_owned()),
        Page::PerfectArchive => pages::archive::perfect::render(&model),
        Page::PrimalityChecker => pages::primalitytest::render(&model),
        Page::PrimeNumbersArchive => pages::archive::prime::render(model.slug.to_owned()),
    }
}

fn routes(url: seed::Url) -> (Page, std::string::String) {
    let empty_string = "".to_owned();

    if url.path().is_empty() {
        return (Page::Home, empty_string);
    }

    match url.path()[0].as_ref() {
        "archive" => {
            // Determine if we are at the archive page, or a subpage
            match url.path().get(1) {
                None => (Page::Archive, empty_string),
                Some(_) => match url.path()[1].as_ref() {
                    "fermat" => (Page::FermatArchive, empty_string),
                    "fibonacci" => match url.path().get(2).as_ref() {
                        Some(_slug) => (Page::FibonacciArchive, url.path()[2].to_owned()),
                        None => (Page::FibonacciArchive, "1".to_owned()),
                    },
                    "mersenne" => (Page::MersenneArchive, empty_string),
                    "perfect" => (Page::PerfectArchive, empty_string),
                    "prime" => match url.path().get(2).as_ref() {
                        Some(_slug) => (Page::PrimeNumbersArchive, url.path()[2].to_owned()),
                        None => (Page::PrimeNumbersArchive, "1".to_owned()),
                    },
                    _ => (Page::Error, empty_string),
                },
            }
        }
        "contact" => (Page::Contact, empty_string),
        "cruncher" => match url.path().get(1).as_ref() {
            Some(_slug) => (Page::NumberCruncher, url.path()[1].to_owned()),
            None => (Page::NumberCruncher, empty_string),
        },
        "faq" => (Page::Faq, empty_string),
        "primalitytest" => (Page::PrimalityChecker, url.path()[0].to_owned()),
        _ => (Page::Error, empty_string),
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    App::start("app", init, update, view);
}
