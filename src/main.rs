extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate dotenv;
extern crate rand;
extern crate rand_xorshift;


use std::{env};
use rand::rngs::SmallRng;
use rand_xorshift::XorShiftRng;
use rand::seq::IteratorRandom;
use rand::{SeedableRng};
use std::collections::{HashMap, HashSet};
use dotenv::dotenv;
use actix_web::{server, App, fs, Result, HttpRequest, Json};
use std::cell::RefCell;

#[derive(Debug, Serialize, Deserialize)]
struct BowlingPin {
    id: u32,
    user_id: u32,
    name: String,
    image: Option<String>
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
        let mut rng = XorShiftRng::seed_from_u64(id as u64);
        let pin_persons: Vec<_> = {
            let mut all: Vec<_> = state.persons.values().collect();
            all.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
            all.into_iter().choose_multiple(&mut rng, 10).into_iter().map(|p| p.clone()).collect()
        };
        let pins = pin_persons.iter().enumerate().map(|(i, p)|
            BowlingPin { id: i as u32, user_id: p.id, name: p.name.clone(), image: p.avatar.clone() 
        }).collect();
        let skill_set: HashSet<_> = pin_persons.iter()
            .flat_map(|p| state.skills_by_person_id(p.id).into_iter()
                      .map(|s| (s.id, s.name.clone())))
            .collect();
        let throws = skill_set.into_iter()
            .map(|(id, name)| BowlingThrow { id, name })
            .collect();
        BowlingGame { id: id, pins, fallen: Vec::default(), throws }
    }
    fn play(&mut self, throw: u32, state: &AppState) {
        self.throws.retain(|t| t.id != throw);
        let falling: Vec<_> = self.pins.iter().filter(|p| !self.fallen.contains(&p.id))
            .filter(|p| state.skills_by_person_id(p.user_id).into_iter().any(|s| s.id == throw))
            .map(|s| s.id).collect();
        self.fallen.extend(falling.iter());
    }
}

struct AppState {
    persons: HashMap<u32, Person>,
    skills: RefCell<HashMap<u32, Vec<Skill>>>
}

impl AppState {
    fn new() -> Self {
        let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
        let persons = Hohto::new(&session_cookie).persons().unwrap();
        AppState {
            persons: persons.items.into_iter().map(|p| (p.id, p)).collect(),
            skills: HashMap::default().into()
        }
    }
    fn skills_by_person_id(&self, id: u32) -> Vec<Skill> {
        let mut skills = self.skills.borrow_mut();
        match skills.get(&id) {
            Some(skills) => skills.clone(),
            None => {
                let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
                let person_skills: Vec<_> = Hohto::new(&session_cookie).skills_by_person_id(id)
                    .unwrap().items.into_iter()
                    .map(|ps| Skill { id: ps.id, name: ps.name.fi }).collect();
                skills.insert(id, person_skills.clone());
                person_skills
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    id: u32,
    name: String,
    avatar: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Persons {
    items: Vec<Person>
}

#[derive(Clone)]
struct Skill {
    id: u32,
    name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SkillName {
    fi: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PersonSkill {
    id: u32,
    name: SkillName
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PersonSkills {
    items: Vec<PersonSkill>
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

  fn skills_by_person_id(&self, id: u32) -> Result<PersonSkills, ()> {
    let url = format!("https://hohtopp.goforecrew.com/api/persons/{}/skills", id);
    reqwest::Client::new()
        .get(&url)
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
fn skills(_req: &HttpRequest<AppState>) -> Result<Json<Vec<PersonSkill>>> {
    let session_cookie = env::var("HOHTO_SESSION").expect("Expected HOHTO_SESSION environment variable");
    let skills = Hohto::new(&session_cookie).skills_by_person_id(272).unwrap();
    Ok(Json(skills.items))
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

    server::new(|| App::with_state(AppState::new())
                .resource("/api/me", |r| r.f(me))
                .resource("/api/new_game", |r| r.f(new_game))
                .resource("/api/play", |r| r.with(play))
                .resource("api/bowling_pins", |r| r.f(bowling_pins))
                .resource("api/skills", |r| r.f(skills))
		.resource("/", |r| r.f(index))
		.handler("/", fs::StaticFiles::new("static").unwrap()))
		.bind("127.0.0.1:8080").unwrap()
		.run();
}
