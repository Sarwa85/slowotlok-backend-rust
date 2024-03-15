use crate::card;
use crate::card::Card;
use crate::io;
use crate::messages;
use crate::repo::Repository;
use crate::repo::RepositorySimpleResult;
use colored::Colorize;

pub struct Logic {
    repo: Repository,
}

impl Logic {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub fn start(&mut self) {
        loop {
            messages::print_title("Słowotłok");
            println!(
                "Words count: {}",
                format!("{}", self.repo.count()).black().on_white()
            );
            let choice = Logic::ask_for_list(vec!["Test (random)", "Manage words"]);
            match choice.trim() {
                "1" => {
                    self.play_random_cards();
                }
                "2" => {
                    self.word_manage();
                }
                "q" => {
                    return;
                }
                _ => messages::print_wrong_choice(&choice[..]),
            }
        }
    }

    pub fn play_random_cards(&mut self) {
        messages::print_title("Test (random)");
        let cards = self.repo.random(10);
        for (index, card) in cards.iter().enumerate() {
            self.check_card(&card, index, cards.len());
        }
    }

    pub fn word_manage(&mut self) {
        messages::print_title("Manage words");
        let choice = Logic::ask_for_list(vec!["Add", "Remove"]);
        match choice.trim() {
            "1" => self.word_add(),
            "2" => self.word_rm(),
            _ => (),
        }
    }

    pub fn word_add(&mut self) {
        messages::print_title("Adding new word");
        loop {
            let source = Logic::ask_for("Source");
            let translation = Logic::ask_for("Translation");
            let choice = Logic::ask_for("[t/n] Is it OK?");
            match choice.trim() {
                "t" => {
                    let card = Card::new(source, translation);
                    self.repo.insert(card);
                    break;
                }
                _ => (),
            }
        }
    }

    pub fn word_rm(&mut self) {
        let cards = self.repo.all();
        let elements: Vec<String> = cards
            .iter()
            .map(|e| format!("{} {}", e.source, e.translation))
            .collect();
        loop {
            let res = Logic::ask_for_list(elements.iter().map(|e| e.as_ref()).collect());
            if res.trim() == "q" {
                return;
            }
            // TODO do it better
            let index: usize = res.trim().parse().unwrap();
            if index >= 1 && index <= elements.len() {
                println!("It's OK {:#?}", cards.get(index - 1).unwrap());
                // remove
                return;
            } else {
                continue;
            }
        }
    }

    pub fn ask_for(text: &str) -> String {
        println!("{text}");
        io::read_line().trim().to_string()
    }

    pub fn ask_for_list(options: Vec<&str>) -> String {
        let opt_size = options.len();
        loop {
            for (index, option) in options.iter().enumerate() {
                println!("{}. {option}", index + 1)
            }
            println!("Choose [{}-{}] or [q] to go back", 1, opt_size);
            let choice = io::read_line();
            if choice.trim() == "q" {
                return choice;
            }
            let num = choice.trim().parse::<usize>();
            match num {
                Ok(n) => {
                    if (0..opt_size + 1).contains(&n) {
                        return choice;
                    }
                }
                Err(_) => (),
            }
        }
    }

    fn check_card(&mut self, card: &card::Card, index: usize, count: usize) {
        messages::print_title(format!("Test {}/{}", index + 1, count).as_str());
        println!("Source: {}", card.source.black().on_white());
        messages::print_press_enter();
        io::read_line();
        println!("Translation: {}", card.translation.black().on_white());
        loop {
            messages::print_voting();
            let choice = io::read_line();
            match choice.trim() {
                "+" => {
                    let mut card = card.clone();
                    card.good += 1;
                    let result = self.repo.update(&card);
                    match result {
                        RepositorySimpleResult::OK => {
                            println!("Database updated");
                            break;
                        }
                        RepositorySimpleResult::Failed(_) => todo!(),
                    }
                }
                "-" => {
                    let mut card = card.clone();
                    card.bad += 1;
                    let result = self.repo.update(&card);
                    match result {
                        RepositorySimpleResult::OK => {
                            println!("Database updated");
                            break;
                        }
                        RepositorySimpleResult::Failed(_) => todo!(),
                    }
                }
                _ => messages::print_wrong_choice(&choice[..]),
            }
        }
    }
}
