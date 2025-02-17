use rsrusl;

pub fn run() {
  rsrusl::tests! {
    rsrusl::log("tip", "This is a tip.");
    rsrusl::log("good", "This is good.");
    rsrusl::log("warning", "This is a warning.");
    rsrusl::log("error", "This is an error.");
    rsrusl::log("userhelp", "This is the userhelp color.");
    rsrusl::log("test", "This is the test color.");
    rsrusl::log("", "This is the default color.");
  }
}
