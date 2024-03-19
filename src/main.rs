use std::rc::Rc;
use slowotlok_backend_rust::repo::{Repository};

mod ui;

fn main() {
    let repo = Repository::new();
    let rc_repo = Rc::new(repo);
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    ui::show_start(&mut *siv, rc_repo.clone());
    siv.run();
}
