use crate::Msg;
use chrono::{NaiveDate};
use seed::prelude::*;

const NUM_NEWS_ITEMS :usize = 1;

struct NewsItem {
    pub title: String,
    pub date: NaiveDate,
}

fn get_news_items() -> [NewsItem; NUM_NEWS_ITEMS] {
    let news_items: [NewsItem; NUM_NEWS_ITEMS] = [
        NewsItem{
            title: String::from(
                "<a class=\"link\" href=\"https://github.com/craigmayhew/ethereum-contracts\">Ethereum bounty contracts</a>have now been added for anyone that would like to try to push the boundaries of human knowledge. The first contract is a competition for the correct solution to a³+b³+c³ = 33. Tweet me if you need assistance using these contracts!"),
            date: NaiveDate::from_ymd(2019, 1, 22)
        }
    ];
    news_items
}

pub fn render() -> seed::dom_types::Node<Msg> {
    let news_items = get_news_items();
    let mut table_rows = vec![div![]; NUM_NEWS_ITEMS];
    for i in 0..news_items.len() {
        let news_item = &news_items[i];
        table_rows[i] = tr![
            td![news_item.date.to_string()],
            td![news_item.title]
        ];
    };
    
    table![table_rows]
}