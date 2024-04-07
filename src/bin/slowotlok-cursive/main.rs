use std::{cell::RefCell, rc::Rc};
use slowotlok_backend::simple_repository::{SimpleRepository};

mod ui;

fn main() {
    // let repo = Repository::new();
    let rc_repo = Rc::new(RefCell::new(SimpleRepository::new()));
    let mut siv = cursive::default();
    // siv.add_global_callback('q', |s| s.quit());
    ui::show_start(&mut *siv, rc_repo);
    siv.run();
}
