pub mod manage_words;
pub mod test;

use std::{cell::RefCell, rc::Rc};

use cursive::{
    view::Resizable,
    views::{Button, Dialog, LinearLayout},
    Cursive,
};
pub use manage_words::show_manage_words;
use slowotlok_backend::{card::Card, simple_repository::SimpleRepository};
pub use test::show_test_random;

#[derive(Clone, Debug)]
pub struct VoteData {
    card: Card,
    index: usize,
    max_index: usize,
}

pub fn show_start(s: &mut Cursive, repo: Rc<RefCell<SimpleRepository>>) {
    s.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Test (random)", {
                    let repo = Rc::clone(&repo);
                    move |s| show_test_random(s, Rc::clone(&repo))
                }))
                .child(Button::new("Manage words", {
                    let repo = Rc::clone(&repo);
                    move |s| show_manage_words(s, Rc::clone(&repo))
                }))
                .child(Button::new("Quit", |s| s.quit())),
        )
        .title("Słowotłok")
        .full_screen(),
    );
}

pub fn show_message(s: &mut Cursive, text: &str) {
    s.add_layer(Dialog::text(text).button("OK", |s| {s.pop_layer();}))
}

pub fn show_card_details(s: &mut Cursive, card: &Card) {
    s.add_layer(Dialog::text(format!("{} / {} / {} / {}", card.source, card.translation, card.good, card.bad)).button("OK", |s| {s.pop_layer();}))
}