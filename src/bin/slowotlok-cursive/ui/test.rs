use std::{cell::RefCell, rc::Rc};

use cursive::{
    event,
    views::{Dialog, OnEventView},
    Cursive,
};
use slowotlok_backend::{card::Card, repo::Repository};

use super::VoteData;

pub fn show_test_random(s: &mut Cursive, repo: Rc<RefCell<Repository>>) {
    let words = repo.borrow().random(10);
    let words_len = words.len();
    for (i, word) in words.iter().enumerate() {
        s.add_layer(
            Dialog::text(format!("{}", word.source))
                .button("Checking", {
                    let word = word.clone();
                    let repo = Rc::clone(&repo);
                    move |s| {
                        show_voting(
                            s,
                            Rc::clone(&repo),
                            VoteData {
                                card: word.clone(),
                                index: i,
                                max_index: words_len,
                            },
                        )
                    }
                })
                .title(format!("Test {}/{}", words_len - i, words_len)), // .full_screen(),
        );
    }
}

fn show_voting(s: &mut Cursive, repo: Rc<RefCell<Repository>>, vote_data: VoteData) {
    s.pop_layer();
    // let repo_good_event = Rc::clone(&repo);
    // let repo_bad_event = Rc::clone(&repo);
    s.add_layer(
        OnEventView::new(
            Dialog::text(format!(
                "Vote for'{}' '{}'\n\nPress <Up>|<Down> for voting",
                vote_data.card.source, vote_data.card.translation
            ))
            .button("+", {
                let repo = Rc::clone(&repo);
                let card = vote_data.card.clone();
                move |s| vote(s, Rc::clone(&repo), card.clone(), true)
            })
            .button("-", {
                let repo = Rc::clone(&repo);
                let card = vote_data.card.clone();
                move |s| vote(s, Rc::clone(&repo), card.clone(), false)
            })
            .title(format!(
                "Vote {}/{}",
                vote_data.max_index - vote_data.index,
                vote_data.max_index
            )),
        )
        .on_event(event::Key::Up, {
            let repo = Rc::clone(&repo);
            let card = vote_data.card.clone();
            move |s| vote(s, Rc::clone(&repo), card.clone(), true)
        })
        .on_event(event::Key::Down, {
            let repo = Rc::clone(&repo);
            let card = vote_data.card.clone();
            move |s| vote(s, Rc::clone(&repo), card.clone(), false)
        }),
    );
}

fn vote(s: &mut Cursive, repo: Rc<RefCell<Repository>>, mut card: Card, good: bool) {
    if good {
        card.good += 1
    } else {
        card.bad += 1
    }
    // repo
    match (*repo).borrow_mut().update(&card) {
        slowotlok_backend::repo::RepositorySimpleResult::OK => (),
        slowotlok_backend::repo::RepositorySimpleResult::Failed(_) => todo!(),
    }
    s.pop_layer();
}
