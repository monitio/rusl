use rsrusl;

pub fn run() {
    rsrusl::tests! {
        let input = rsrusl::i::userinput("What is your name?");
        rsrusl::g(&input)
    }
}
