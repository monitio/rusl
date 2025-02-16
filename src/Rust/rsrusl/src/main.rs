fn main() {
    // Use our rusl library functions
    rsrusl::hw();
    rsrusl::g("Alice");
    let input = rsrusl::userinput("Enter your name: ");
    rsrusl::g(&input);
}
