use seed::prelude::*;
use std::{convert::TryFrom, fmt, borrow::Cow};

type Path<'a> = Vec<&'a str>;

#[derive(Clone)]
pub enum Route<> {
    Home,
    ContactUs,
    Faq,
    Downloads,
    Status,
    NumberCruncher,
    PrimalityChecker,
    PrimeNumbersArchive,
    MersennePrimeArchive,
    FermatArchive,
    PerfectArchive,
    FibonacciArchive
    //Article(article::slug::Slug),
    //Profile(Cow<'a, username::Username<'a>>),
}

impl<'a> Route<> {
    pub fn path(&self) -> Path {
        match self {
            Route::Home => vec![],
            Route::ContactUs => vec!["contactus"],
            Route::Faq => vec!["faq"],
            Route::Downloads => vec!["downloads"],
            Route::Status => vec!["status"],
            Route::NumberCruncher => vec!["NumberCruncher"],
            Route::PrimalityChecker => vec!["PrimalityChecker"],
            Route::PrimeNumbersArchive => vec!["PrimeNumbersArchive"],
            Route::MersennePrimeArchive => vec!["MersennePrimeArchive"],
            Route::FermatArchive => vec!["FermatArchive"],
            Route::PerfectArchive => vec!["PerfectArchive"],
            Route::FibonacciArchive => vec!["FibonacciArchive"],
            //Route::Article(slug) => vec!["article", slug.as_str()],
            //Route::Profile(username) => vec!["profile", username.as_str()],
        }
    }
}

impl<'a> fmt::Display for Route<> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl<'a> From<Route<>> for seed::Url {
    fn from(route: Route) -> Self {
        route.path().into()
    }
}

impl<'a> TryFrom<seed::Url> for Route<> {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("contactus") => Some(Route::ContactUs),
            Some("faq") => Some(Route::Faq),
            /*Some("article") => {
                path
                    .next()
                    .filter(non_empty)
                    .map(article::slug::Slug::from)
                    .map(Route::Article)
            },*/
            _ => None,
        }.ok_or(())
    }
}