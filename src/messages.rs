use colored::Colorize;

pub fn print_wrong_choice(choice: &str) {
    println!("{} {}", "Wrong option".red(), choice.red());
}

pub fn print_choose() {
    println!("{}", "Choose".yellow())
}

pub fn print_press_enter() {
    println!("{}", "Press enter".yellow());
}

pub fn print_title(title: &str) {
    println!("{}", format!("### {title} ###").bold().green());
}

pub fn print_voting() {
    println!("{}", "Choose +/-".yellow());
}

pub fn print_todo(text: &str) {
    println!("{}", text.magenta())
}
