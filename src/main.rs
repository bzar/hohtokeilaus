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
    id: u32,
    name: String,
    image: String
}
#[derive(Debug, Serialize, Deserialize)]
struct BowlingThrow {
    id: u32,
    name: String
}
#[derive(Debug, Serialize, Deserialize)]
struct BowlingGame {
    id: u32,
    pins: Vec<BowlingPin>,
    throws: Vec<BowlingThrow>
}

#[derive(Debug, Serialize, Deserialize)]
struct BowlingPlay {
    game: u32,
    throws: Vec<u32>
}

impl BowlingGame {
    fn from_id(id: u32) -> Self {
        BowlingGame {
            id: id,
            pins: vec![
                BowlingPin { id: 0, name: "foo0".into(), image: "image0".into() },
                BowlingPin { id: 1, name: "foo1".into(), image: "image1".into() },
                BowlingPin { id: 2, name: "foo2".into(), image: "image2".into() },
                BowlingPin { id: 3, name: "foo3".into(), image: "image3".into() },
                BowlingPin { id: 4, name: "foo4".into(), image: "image4".into() }
            ],
            throws: vec![
                BowlingThrow { id: 0, name: "skill0".into() },
                BowlingThrow { id: 1, name: "skill1".into() },
                BowlingThrow { id: 2, name: "skill2".into() },
                BowlingThrow { id: 3, name: "skill3".into() }
            ]
        }
    }
    fn play(&mut self, throw: u32) {
        self.throws.retain(|t| t.id != throw);
    }
}

struct AppState {
    
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    id: usize,
    name: String,
    email: String
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
    let game = BowlingGame::from_id(42);
    Ok(Json(game))
}
fn play(bp: Json<BowlingPlay>) -> Result<Json<BowlingGame>> {
    let mut game = BowlingGame::from_id(bp.game);
    for throw in &bp.throws {
        game.play(*throw);
    }
    Ok(Json(game))
}

pub fn main() {
    dotenv().ok();

    server::new(|| App::new()
                .resource("/api/me", |r| r.f(me))
                .resource("/api/new_game", |r| r.f(new_game))
                .resource("/api/play", |r| r.with(play))
		.resource("/", |r| r.f(index))
		.handler("/", fs::StaticFiles::new("static").unwrap()))
		.bind("127.0.0.1:8080").unwrap()
		.run();
}
