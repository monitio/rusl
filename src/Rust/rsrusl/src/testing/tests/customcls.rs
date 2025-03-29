use rsrusl;

pub fn run() {
    rsrusl::tests! {
      let input = rsrusl::i::userinput("What setting do you want to use? (\"y\"or\"yes\" / \"n\"or\"no\")");
      let testingname = rsrusl::i::userinput("What name are you going to use to test?");
      let testingversion = rsrusl::i::userinput("What version are you going to use for the version?");

      if input == "y" {
        rsrusl::customcls("y", &testingname, &testingversion);
      } else if input == "yes" {
        rsrusl::customcls("yes", &testingname, &testingversion);
      } else if input == "n" {
        rsrusl::customcls("n", "", "");
      } else if input == "no" {
        rsrusl::customcls("no", "", "");
      } else {
        rsrusl::log("error", "Invalid input type for setting.");
      }
    }
}
