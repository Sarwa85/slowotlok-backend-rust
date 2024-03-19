pub mod manage_words;
pub mod test;

use std::rc::Rc;

use cursive::{
    view::Resizable,
    views::{Button, Dialog, LinearLayout},
    Cursive,
};
pub use manage_words::show_manage_words;
use slowotlok_backend_rust::{card::Card, repo::Repository};
pub use test::show_test_random;

#[derive(Clone, Debug)]
pub struct VoteData {
    card: Card,
    index: usize,
    max_index: usize,
}

pub fn show_start(s: &mut Cursive, repo: Rc<Repository>) {
    let repo_start = repo.clone();
    let repo_manage = repo.clone();
    s.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Test (random)", move |s| {
                    show_test_random(s, repo_start.clone());
                }))
                .child(Button::new("Manage words", move |s| {
                    show_manage_words(s, repo_manage.clone());
                }))
                .child(Button::new("Quit", |s| s.quit())),
        )
        .title("Słowotłok")
        .full_screen(),
    );
}
