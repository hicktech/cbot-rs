use soup::{NodeExt, QueryBuilderExt, Soup};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Board {
    pub corn: u64,
    pub soybean: u64,
    pub wheat: u64,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\tcorn: {}\n\tsoybean: {}\n\twheat: {}",
            self.corn as f64 / 100.0,
            self.soybean as f64 / 100.0,
            self.wheat as f64 / 100.0
        ))
    }
}

#[derive(Debug)]
pub struct Quote {
    pub month: String,
    pub price: u64,
    pub change: i64,
    pub time: String,
}

pub fn extract_quotes(crop: &str, soup: &Soup) -> Vec<Quote> {
    let attr = format!("grains-{crop}-quote-table");
    let t = soup.tag("table").attr("id", attr).find().unwrap();

    let b = t.tag("tbody").find().unwrap();

    b.tag("tr")
        .find_all()
        .map(|tr| {
            let month = tr.class("text-left").find().unwrap().text();
            let price = tr.class("price").find().unwrap().text();
            let change = tr.class("quoteEven").find().unwrap().text();
            let time = tr.class("text-right").find_all().last().unwrap().text();

            let (price, _) = price.split_once('-').unwrap();
            let (change, _) = change.rsplit_once('-').unwrap();

            Quote {
                month,
                price: price.parse().unwrap(),
                change: change.parse().unwrap(),
                time,
            }
        })
        .collect()
}
