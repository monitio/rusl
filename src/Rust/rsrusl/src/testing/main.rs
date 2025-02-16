// Rusl Testing. - 0.1.0
// Use our rusl library functions

use rsrusl;

fn main() {
    rsrusl::testing::run_tests(|| {
        userinput();
        // You can add more tests here
    });
}

fn userinput() {
    // Use our rusl library functions
    let input = rsrusl::i::userinput("Enter your name:");
    rsrusl::g(&input);
}
