#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use seed::prelude::*;

pub mod pages;

#[derive(Copy, Clone)]
pub enum Page {
    ContactUs,
    Downloads,
    Faq,
    Home,
    Status,
    /*
    NumberCruncher,
    PrimalityChecker,
    PrimeNumbersArchive,
    MersennePrimeArchive,
    FermatArchive,
    PerfectArchive,
    FibonacciArchive*/
    //Article(article::slug::Slug),
}

// Model
struct Model {
    page: Page,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Home,
        }
    }
}

// Update
#[derive(Clone)]
pub enum Msg {
    ChangePage(Page),
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page
    }
}

/// The top-level component we pass to the virtual dom. Must accept the model as its
/// only argument, and output has to implement trait `ElContainer`.
fn view(model: &Model) -> impl View<Msg> {
    match model.page {
        Page::ContactUs => pages::contact::render(),
        Page::Downloads => pages::downloads::render(),
        Page::Faq => pages::faq::render(),
        Page::Home => pages::home::render(),
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
            Page::Home => "home".into(),
            Page::Status => "status".into(),
        }
    }
}

fn routes(url: seed::Url) -> Msg {
    if url.path.is_empty() {
        return Msg::ChangePage(Page::Home)
    }

    match url.path[0].as_ref() {
        "guide" => {
            // Determine if we are at the main page, or a subpage
            match url.path.get(1).as_ref() {
                Some(_page) => Msg::ChangePage(Page::Home),
                None => Msg::ChangePage(Page::Home)
            }
        },
        "contactus" => Msg::ChangePage(Page::ContactUs),
        "downloads" => Msg::ChangePage(Page::Downloads),
        "faq" => Msg::ChangePage(Page::Faq),
        "status" => Msg::ChangePage(Page::Status),
        _ => Msg::ChangePage(Page::Home)
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .mount("app")
        .routes(routes)
        //.window_events(window_events)
        .finish()
        .run();
}