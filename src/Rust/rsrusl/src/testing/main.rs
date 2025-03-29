// Rusl Testing. - 0.1.3
// Use our rusl library functions

use rsrusl;

#[path = "tests/logging.rs"]
mod logging;

#[path = "tests/userinput.rs"]
mod userinput;

#[path = "tests/cls.rs"]
mod cls;

#[path = "tests/customcls.rs"]
mod customcls;

fn testslist() {
    rsrusl::customcls("yes", "Rusl Testing", "0.1.3");
    rsrusl::v();

    rsrusl::log("tip", "cls");
    rsrusl::log("tip", "logging");
    rsrusl::log("tip", "customcls");
    rsrusl::log("tip", "userinput");
}

fn main() {
    testslist();
    println!("\n----\n");
    let input = rsrusl::i::userinput("Which test do you want to run from the list above?");
    if input.trim().is_empty() {
        rsrusl::log("error", "No test provided. Please enter a valid test.");
        return; // Optionally exit early.
    }
    if input == "logging" {
        logging::run();
    } else if input == "userinput" {
        userinput::run();
    } else if input == "cls" {
        cls::run();
    } else if input == "customcls" {
        customcls::run();
    } else {
        rsrusl::log("error", "Unknown test provided. Please enter a valid test.");
    }
}
