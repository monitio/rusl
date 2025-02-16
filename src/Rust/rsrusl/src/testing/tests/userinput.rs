use rsrusl;

fn main() {
    testing::run_tests(|| {
        userinput();
        // You can add more tests here
    });
}

fn userinput() {
    // Use our rusl library functions
    let input = rsrusl::i::userinput("Enter your name:");
    rsrusl::g(&input);
}
