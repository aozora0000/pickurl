extern crate regex;
use std::env;
use seahorse::{App, Context};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("url pickup from stdin")
        .action(action);
    app.run(args);
}

fn action(_c: &Context) {
    let regex = Regex::new(r"https?://[\w!\?/\+\-_~=;\.,\*&@#\$%\(\)'\[\]]+").unwrap();
    for line in std::io::stdin().lines() {
        match line {
            Err(_) => break,    // with ^Z
            Ok(s) => {
                for url in regex.captures(&s).iter() {
                    println!("{}", url.get(0).unwrap().as_str());
                }
            },
        }
    }
}