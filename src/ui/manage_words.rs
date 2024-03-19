use std::rc::Rc;

use cursive::{view::Resizable, views::{Button, Dialog, LinearLayout}, Cursive};
use slowotlok_backend_rust::repo::Repository;

// pub mod manage_words {
pub fn show_manage_words(s: &mut Cursive, repo: Rc<Repository>) {
    s.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("Quick add", move |s| {
                    // show_test_random(s, repo_start.clone());
                }))
                .child(Button::new("List", move |s| {
                    // show_manage_words(s, repo_manage.clone());
                }))
                .child(Button::new("Back", |s| {
                    s.pop_layer();
                })),
        )
        .title("Słowotłok 1")
        .full_screen(),
    );
}
// }
