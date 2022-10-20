use std::{env, process::exit};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Bank {
    #[serde(rename(deserialize = "BANK"))]
    bank: String,
    #[serde(rename(deserialize = "BRANCH"))]
    branch: String,
    #[serde(rename(deserialize = "ADDRESS"))]
    address: String,
    #[serde(rename(deserialize = "CITY"))]
    city: String,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.get(1).is_none() {
        println!("no arg passed, need at least one, IFSC");
        exit(1)
    }

    let response = reqwest::blocking::get(format!(
        "https://ifsc.razorpay.com/{}",
        args.get(1).unwrap()
    ));

    let bank = response
        .and_then(|res| res.json::<Bank>())
        .expect("unable to parse response");

    println!("BANK: {}", bank.bank);
    println!("BRANCH: {}", bank.branch);
    println!("ADDRESS: {}", bank.address);
    println!("CITY: {}", bank.city);
}
