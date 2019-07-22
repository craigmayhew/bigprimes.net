pub mod Home {
    use chrono::{NaiveDate};

    pub struct NewsItem {
        pub title: String,
        pub date: NaiveDate,
    }

    fn changeFuncName() -> NewsItem {
        let item :NewsItem = NewsItem{
                                title: String::from("<a class=\"link\" href=\"https://github.com/craigmayhew/ethereum-contracts\">Ethereum bounty contracts</a> have now been added for anyone that would like to try to push the boundaries of human knowledge. The first contract is a competition for the correct solution to a³+b³+c³ = 33. Tweet me if you need assistance using these contracts!"),
                                date: NaiveDate::from_ymd(2019, 1, 22)
                            };
        item
    }

}