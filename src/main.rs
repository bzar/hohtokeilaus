extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;


use std::{env};
use std::collections::{HashMap, HashSet};
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
    fallen: Vec<u32>,
    throws: Vec<BowlingThrow>
}

#[derive(Debug, Serialize, Deserialize)]
struct BowlingPlay {
    game: u32,
    throws: Vec<u32>
}

impl BowlingGame {
    fn from_id(id: u32, state: &AppState) -> Self {
        let pin_persons: Vec<_> = state.persons.values().take(10).map(|p| p.clone()).collect();
        let pins = pin_persons.iter().enumerate().map(|(i, p)|
            BowlingPin { id: i as u32, name: p.name.clone(), image: p.name.clone() 
        }).collect();
        let skill_set: HashSet<_> = pin_persons.iter()
            .flat_map(|p| state.skills.get(&p.id).unwrap().iter().map(|s| (s.id, s.name.clone())))
            .collect();
        let throws = skill_set.into_iter()
            .map(|(id, name)| BowlingThrow { id, name })
            .collect();
        BowlingGame { id: id, pins, fallen: Vec::default(), throws }
    }
    fn play(&mut self, throw: u32, state: &AppState) {
        self.throws.retain(|t| t.id != throw);
        let falling: Vec<_> = self.pins.iter().filter(|p| !self.fallen.contains(&p.id))
            .filter(|p| state.skills.get(&p.id).unwrap().iter().any(|s| s.id == throw))
            .map(|s| s.id).collect();
        self.fallen.extend(falling.iter());
    }
}

struct AppState {
    persons: HashMap<u32, Person>,
    skills: HashMap<u32, Vec<Skill>>
}

impl Default for AppState {
    fn default() -> Self {
        let mut s = AppState {
            persons: HashMap::default(),
            skills: HashMap::default()
        };
        for pid in 0..128 {
            s.persons.insert(pid, Person {
                id: pid,
                name: format!("Person {}", pid)
            });
        }
        for pid in 0..256 {
            s.skills.insert(pid, vec![
                            Skill { id: 0, name: "Skill 0".into() },
                            Skill { id: 1, name: "Skill 1".into() },
                            Skill { id: 2, name: "Skill 2".into() },
            ]);
        }

        s
    }
}
impl AppState {
    fn new() -> Self {
        let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
        let persons = Hohto::new(&session_cookie).persons().unwrap();
        AppState {
            persons: persons.items.into_iter().map(|p| (p.id, p)).collect(),
            skills: HashMap::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    id: u32,
    name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Persons {
    items: Vec<Person>
}

struct Skill {
    id: u32,
    name: String
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

  fn persons(&self) -> Result<Persons, ()> {
    reqwest::Client::new()
        .get("https://hohtopp.goforecrew.com/api/persons")
        .header(reqwest::header::COOKIE, self.session_cookie.clone())
        .send().or(Err(()))?
        .json().or(Err(()))
  }
}

fn index(_req: &HttpRequest<AppState>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}
fn me(_req: &HttpRequest<AppState>) -> Result<Json<Person>> {
    let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
    let person = Hohto::new(&session_cookie).me().unwrap();
    Ok(Json(person))
}
fn bowling_pins(_req: &HttpRequest<AppState>) -> Result<Json<Vec<Person>>> {
    let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
    let persons = Hohto::new(&session_cookie).persons().unwrap();
    Ok(Json(persons.items))
}
fn new_game(req: &HttpRequest<AppState>) -> Result<Json<BowlingGame>> {
    let game = BowlingGame::from_id(42, req.state());
    Ok(Json(game))
}
fn play((bp, req): (Json<BowlingPlay>, HttpRequest<AppState>)) -> Result<Json<BowlingGame>> {
    let mut game = BowlingGame::from_id(bp.game, req.state());
    for throw in &bp.throws {
        game.play(*throw, req.state());
    }
    Ok(Json(game))
}

pub fn main() {
    dotenv().ok();

    server::new(|| App::with_state(AppState::default())
                .resource("/api/me", |r| r.f(me))
                .resource("/api/new_game", |r| r.f(new_game))
                .resource("/api/play", |r| r.with(play))
                .resource("api/bowling_pins", |r| r.f(bowling_pins))
		.resource("/", |r| r.f(index))
		.handler("/", fs::StaticFiles::new("static").unwrap()))
		.bind("127.0.0.1:8080").unwrap()
		.run();
}
