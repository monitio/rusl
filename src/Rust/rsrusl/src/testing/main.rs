// Rusl Testing. - 0.1.1
// Use our rusl library functions

use rsrusl;

#[path = "tests/logging.rs"]
mod logging;

#[path = "tests/userinput.rs"]
mod userinput;

fn testslist() {
    rsrusl::cls("yes");

    rsrusl::log("tip", "- logging");
    rsrusl::log("tip", "- userinput");
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
    } else {
        rsrusl::log("error", "Unknown test provided. Please enter a valid test.");
    }
}
