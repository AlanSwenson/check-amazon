use reqwest::Error;
use serde::Deserialize;
use std::io;
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    // Name your user agent after your app?
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let mut asin = String::new();
    io::stdin()
        .read_line(&mut asin)
        .expect("Failed to read line");
    println!("asin = {}", asin);

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    println!("{}", request_url);
    let response = client.get(&request_url).send()?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
