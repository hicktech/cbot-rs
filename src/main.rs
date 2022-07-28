use std::env;
use std::error::Error;
use soup::Soup;
use cbot::quotes::{Board, extract_quotes};

fn main() -> Result<(), Box<dyn Error>> {
    let quote_url = env::args().nth(1).expect("usage: main <quote-url>");
    let resp = reqwest::get(&quote_url)?;
    let soup = Soup::from_reader(resp)?;

    let corn = extract_quotes("corn", &soup)
        .first()
        .map(|x| x.price)
        .unwrap();

    let soybean = extract_quotes("soybean", &soup)
        .first()
        .map(|x| x.price)
        .unwrap();

    let wheat = extract_quotes("wheat", &soup)
        .first()
        .map(|x| x.price)
        .unwrap();

    let prices = Board {
        corn,
        soybean,
        wheat,
    };
    println!("Current board prices:\n{}", prices);

    Ok(())
}
