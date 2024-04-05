use std::{cell::RefCell, rc::Rc};

use cursive::{
    view::{Nameable, Resizable},
    views::{Button, Dialog, LinearLayout, OnEventView, SelectView},
    Cursive,
};
use slowotlok_backend_rust::{card::Card, repo::Repository};

use super::{show_card_details, show_message};

// pub mod manage_words {
pub fn show_manage_words(s: &mut Cursive, repo: Rc<RefCell<Repository>>) {
    let word_list_view = SelectView::<Card>::new()
        .on_submit(|s, c: &Card| show_card_details(s, c))
        .with_name("select")
        .fixed_size((10, 5));
    let back_button = Button::new("Back", |s| {
        s.pop_layer();
    });
    // let footer = TextView::new("Delete <d>").align(Align::center());

    s.add_fullscreen_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(word_list_view)
                    // .child(footer)
                    .child(back_button)
            )
            .title("Słowotłok 1")
            .full_screen(),
        )
        .on_event('d', |s| show_message(s, "delete")),
    );

    for (index, card) in (*repo).borrow().all().iter().enumerate() {
        s.call_on_name("select", |view: &mut SelectView<Card>| {
            view.add_item(
                format!("{}. {} / {}", index + 1, card.source, card.translation),
                card.clone(),
            )
        });
    }
}
