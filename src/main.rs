extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;


use std::{env};
use dotenv::dotenv;
use actix_web::{server, App, fs, Result, HttpRequest, Json};

#[derive(Debug, Serialize, Deserialize)]
struct BowlingPin {
    name: String,
    image: String
}
#[derive(Debug, Serialize, Deserialize)]
struct BowlingThrow {
    name: String
}
#[derive(Debug, Serialize, Deserialize)]
struct BowlingGame {
    id: u32,
    pins: Vec<BowlingPin>,
    throws: Vec<BowlingThrow>
}

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
fn new_game(_req: &HttpRequest) -> Result<Json<BowlingGame>> {
    let game = BowlingGame {
        id: 42,
        pins: vec![
            BowlingPin { name: "foo0".into(), image: "image0".into() },
            BowlingPin { name: "foo1".into(), image: "image1".into() },
            BowlingPin { name: "foo2".into(), image: "image2".into() },
            BowlingPin { name: "foo3".into(), image: "image3".into() },
            BowlingPin { name: "foo4".into(), image: "image4".into() }
        ],
        throws: vec![
            BowlingThrow { name: "skill0".into() },
            BowlingThrow { name: "skill1".into() },
            BowlingThrow { name: "skill2".into() },
            BowlingThrow { name: "skill3".into() }
        ]
    };
    Ok(Json(game))
}

pub fn main() {
    dotenv().ok();

    server::new(|| App::new()
                .resource("/api/me", |r| r.f(me))
                .resource("/api/new_game", |r| r.f(new_game))
		.resource("/", |r| r.f(index))
		.handler("/", fs::StaticFiles::new("static").unwrap()))
		.bind("127.0.0.1:8080").unwrap()
		.run();

/*
        */
}
