use slowotlok_backend_rust::logic::Logic;
use slowotlok_backend_rust::repo::Repository;

fn main() {
    let repo = Repository::new();
    let mut l = Logic::new(repo);
    l.start();
}
