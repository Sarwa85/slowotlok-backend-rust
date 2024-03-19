use std::{borrow::BorrowMut, rc::Rc};

use cursive::{
    event,
    views::{Dialog, OnEventView},
    Cursive,
};
use slowotlok_backend_rust::{card::Card, repo::Repository};

use super::VoteData;

pub fn show_test_random(s: &mut Cursive, repo: Rc<Repository>) {
    let words = repo.random(10);
    for (i, word) in words.iter().enumerate() {
        let repo_voting = repo.clone();
        let word_copy = word.clone();
        let words_len = words.len();
        s.add_layer(
            Dialog::text(format!("{}", word_copy.source))
                .button("Checking", move |s| {
                    let data = VoteData {
                        card: word_copy.clone(),
                        index: i,
                        max_index: words_len,
                    };
                    show_voting(s, repo_voting.clone(), data);
                })
                .title(format!("Test {}/{}", words.len() - i, words.len())), // .full_screen(),
        );
    }
}

fn show_voting(s: &mut Cursive, repo: Rc<Repository>, vote_data: VoteData) {
    s.pop_layer();
    let repo_good = repo.clone();
    let repo_bad = repo.clone();
    let vote_data_bad = vote_data.clone();

    let repo_good_event = repo.clone();
    let vote_data_event_up = vote_data.clone();
    s.add_layer(
        OnEventView::new(
            Dialog::text(format!(
                "Vote for'{}' '{}'\nPress <Up>|<Down> for voting",
                vote_data.card.source, vote_data.card.translation
            ))
            .button("+", move |s| {
                vote(s, repo_good.clone(), vote_data.card.clone(), true);
            })
            .button("-", move |s| {
                vote(s, repo_bad.clone(), vote_data_bad.card.clone(), true);
            })
            .title(format!("Vote")),
        )
        .on_event(event::Key::Up, move |s| {
            vote(
                s,
                repo_good_event.clone(),
                vote_data_event_up.card.clone(),
                true,
            );
        }),
    );
}

fn vote(s: &mut Cursive, repo: Rc<Repository>, card: Card, good: bool) {
    s.pop_layer();
}
