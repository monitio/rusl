use rsrusl;

pub fn run() {
    rsrusl::tests! {
      let input = rsrusl::i::userinput("What setting do you want to use? (\"y\"or\"yes\" / \"n\"or\"no\")");

      if input == "y" {
        rsrusl::cls("y");
      } else if input == "yes" {
        rsrusl::cls("yes");
      } else if input == "n" {
        rsrusl::cls("n");
      } else if input == "n" {
        rsrusl::cls("no");
      } else {
        rsrusl::log("error", "Invalid input type for setting.");
      }
    }
}
