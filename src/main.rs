#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;
use tokio::{fs::File};
use std::{io, path::PathBuf};

#[derive(Clone, Debug)]
struct App;

#[derive(Serialize,Deserialize,Response)]
struct Person {
    id: usize,
    name: String,
    email: String,
    phone: String
}

const SESSION_COOKIE: &str = "session_id=hohto-session-id-here";

impl_web! {
    impl App {
        #[get("/")]
        #[content_type("text/html")]
        fn index(&self) -> impl Future<Item = File, Error = io::Error> {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("static");
            path.push("index.html");
            File::open(path)
        }

        #[get("/static/*relative_path")]
        #[content_type("plain")]
        fn files(&self, relative_path: PathBuf) -> impl Future<Item = File, Error = io::Error> {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("static");
            path.push(relative_path);
            File::open(path)
        }
        #[get("/api/me")]
        fn me(&self) -> Result<Person, ()> {
            reqwest::Client::new()
                .get("https://hohtopp.goforecrew.com/api/persons/me")
                .header(reqwest::header::COOKIE, SESSION_COOKIE)
                .send().or(Err(()))?
                .json().or(Err(()))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(App)
        .run(&addr)
        .unwrap();
}
