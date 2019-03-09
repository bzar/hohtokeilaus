extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;


use std::{env};
use dotenv::dotenv;
use actix_web::{server, App, fs, Result, HttpRequest, Json};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: usize,
    name: String,
    email: String,
    phone: String
}
struct Hohto {
  session_cookie: String
}

impl Hohto {
  fn new(session_cookie: &str) -> Self {
      Hohto {
          session_cookie: session_cookie.into()
      }
  }
  fn me(&self) -> Result<Person, ()> {
    reqwest::Client::new()
        .get("https://hohtopp.goforecrew.com/api/persons/me")
        .header(reqwest::header::COOKIE, self.session_cookie.clone())
        .send().or(Err(()))?
        .json().or(Err(()))
  }
}

fn index(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}
fn me(_req: &HttpRequest) -> Result<Json<Person>> {
    let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
    let person = Hohto::new(&session_cookie).me().unwrap();
    Ok(Json(person))
}

pub fn main() {
    dotenv().ok();

    server::new(|| App::new()
                .resource("/api/me", |r| r.f(me))
		.resource("/", |r| r.f(index))
		.handler("/", fs::StaticFiles::new("static").unwrap()))
		.bind("127.0.0.1:8080").unwrap()
		.run();

/*
        */
}
