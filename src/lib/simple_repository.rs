use crate::card::Card;
use crate::repository::RepositoryTrait;
use rand::seq::index::sample;
use rand::thread_rng;
use std::cmp;
use std::fs::File;
use std::io::{Read, Write};

pub struct SimpleRepository {
    model: Vec<Card>,
    last_id: i64,
}

pub enum RepositorySimpleResult {
    OK,
    Failed(String),
}

impl SimpleRepository {
    pub fn new() -> Self {
        let mut out = Self {
            model: vec![],
            last_id: -1,
        };
        out.load();
        return out;
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.model) {
            Ok(text) => {
                let mut f = File::create("data.json").unwrap();
                f.write_all(text.as_bytes()).unwrap();
                print!("{text}");
            }
            Err(_) => todo!(),
        }
    }

    fn load(&mut self) {
        let mut f = File::open("data.json").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        self.model = serde_json::from_str(&buf).unwrap();
        match self.model.iter().map(|c| c.id).max() {
            Some(v) => self.last_id = v,
            None => self.last_id = 0,
        }
    }

    fn new_id(&mut self) -> i64 {
        self.last_id += 1;
        self.last_id
    }
}

impl RepositoryTrait for SimpleRepository {
    fn random(&self, count: usize) -> Vec<Card> {
        let mut rng = thread_rng();
        let out = sample(
            &mut rng,
            self.model.len(),
            cmp::min(count, self.model.len()),
        )
        .iter()
        .map(|i| self.model[i].clone())
        .collect();
        out
    }

    fn count(&self) -> usize {
        return self.model.len();
    }

    fn all(&self) -> Vec<Card> {
        self.model.iter().map(|e| e.clone()).collect()
    }

    fn insert(&mut self, card: &mut Card) -> RepositorySimpleResult {
        card.id = self.new_id();
        self.model.push(card.clone());
        self.save();
        RepositorySimpleResult::OK
    }

    fn insert_list(&mut self, cards: &mut Vec<Card>) -> RepositorySimpleResult {
        for card in cards.iter_mut() {
            card.id = self.new_id();
            self.model.push(card.clone());
        }
        self.save();
        RepositorySimpleResult::OK
    }

    fn update(&mut self, card: &Card) -> RepositorySimpleResult {
        match self.model.iter_mut().find(|cd| cd.id == card.id) {
            Some(some) => {
                *some = Card {
                    id: card.id,
                    source: String::from(&card.source),
                    translation: String::from(&card.translation),
                    good: card.good,
                    bad: card.bad,
                };
                self.save();
                RepositorySimpleResult::OK
            }
            None => RepositorySimpleResult::Failed(format!("None card found for id={}", card.id)),
        }
    }

    fn delete(&mut self, card: &Card) -> RepositorySimpleResult {
        match self.model.iter().position(|c| c.id == card.id) {
            Some(i) => {
                self.model.remove(i);
                return RepositorySimpleResult::OK;
            }
            None => {
                return RepositorySimpleResult::Failed(format!("Can't find card"));
            }
        }
    }

    fn delete_by_id(&mut self, id: i64) -> RepositorySimpleResult {
        match self.model.iter().position(|c| c.id == id) {
            Some(i) => {
                self.model.remove(i);
                return RepositorySimpleResult::OK;
            }
            None => {
                return RepositorySimpleResult::Failed(format!("Can't find card"));
            }
        }
    }
}