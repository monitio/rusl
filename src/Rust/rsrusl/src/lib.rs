use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::{io, process::Command};

/// A single new-line character.
pub const NLC: &str = "\n";

/// The current rusl version.
pub const RUSLVER: &str = "0.1.3\n";

/// Prints the current rusl version.
pub fn v() {
    println!("rusl - {}", RUSLVER);
}

/// Prints a new-line character.
pub fn nl() {
    println!("{}", NLC);
}

/// Prints "Hello, World!"
pub fn hw() {
    println!("Hello, World!");
}

/// Clears the console with an option to print the version of rusl being used.
pub fn cls(printversion: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }

    // Validate input and act accordingly
    match printversion.trim().to_lowercase().as_str() {
        "yes" | "y" => crate::v(),
        "no" | "n" => {}
        _ => panic!("Invalid argument for rsrusl::cls(printversion). Use 'yes' or 'no'."),
    }
}

/// Logs a message with a colored "▼" symbol/indicator based on status type.
pub fn log(status: &str, message: &str) {
    let mut stdout = io::stdout();

    // Define colors for each status
    let status_color = match status {
        "tip" => Color::Magenta,
        "test" => Color::Blue,
        "good" => Color::Green,
        "error" => Color::Red,
        "warning" => Color::DarkYellow, // Yellowish/Orange
        "userhelp" => Color::Cyan,
        _ => Color::White, // Default color for unknown statuses
    };

    let formattedmessage = format!("| {}\n", message);

    execute!(
        stdout,
        SetForegroundColor(status_color),
        Print("▼ "),
        ResetColor,
        Print(formattedmessage),
    )
    .unwrap();
}

/// Greets a given name in a random language.
///
/// # Arguments
///
/// * `name` - The name of the person to greet.
pub fn g(name: &str) {
    let greetlanguages = [
        "Hello",
        "Hi",
        "Sup",
        "Howdy",
        "Greetings",
        "Namaste",
        "Bonjour",
        "Salut",
        "Hola",
        "Ciao",
        "Hallo",
        "Hej",
        "Hei",
        "Merhaba",
        "привет",
        "Привіт",
        "Γεια σας",
    ];
    // Use fastrand to generate a random index.
    let idx = fastrand::usize(0..greetlanguages.len());
    println!("{}, {}!", greetlanguages[idx], name);
}

pub mod i {
    use crossterm::{
        execute,
        style::{Color, Print, ResetColor, SetForegroundColor},
    };
    use std::io::{self, Write};

    /// Prompts the user for input and returns the entered string.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt to display to the user.
    pub fn userinput(prompt: &str) -> String {
        let mut stdout = io::stdout();

        // Define colors for each status
        let statuscolor = Color::Cyan;

        let indicator = "🞂 ";
        let formattedprompt = format!("| {} ", prompt);

        execute!(
            stdout,
            SetForegroundColor(statuscolor),
            Print(indicator),
            ResetColor,
            Print(formattedprompt),
        )
        .unwrap();
        io::stdout().flush().expect("Failed to flush stdout");

        let mut useroutput = String::new();
        io::stdin()
            .read_line(&mut useroutput)
            .expect("Failed to read input");

        useroutput.trim().to_string()
    }
}

pub mod testing {
    use crossterm::{
        execute,
        style::{Color, Print, ResetColor, SetForegroundColor},
    };
    use std::{io, panic, time::Instant};

    /// Runs the tests inside the provided closure.
    ///
    /// # DOESN'T WORK:
    /// Any panics inside the test block are caught and suppressed from Cargo's default output.
    /// The indicator color depends on whether an error occurred.
    pub fn run_tests<F>(test_block: F)
    where
        F: FnOnce() + panic::UnwindSafe,
    {
        let start_time = Instant::now();

        // Override the panic hook temporarily so that Cargo doesn't print panic messages.
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(|_info| {
            // Do nothing—suppress panic output.
        }));

        // Run the test block in a catch_unwind context.
        let result = panic::catch_unwind(|| {
            test_block();
        });

        // Restore the original panic hook.
        panic::set_hook(old_hook);

        let elapsed = start_time.elapsed();
        let mut stdout = io::stdout();

        // If the test block panicked, use a red indicator; otherwise, green.
        if result.is_err() {
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                Print("\n▼ "),
                ResetColor
            )
            .unwrap();
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Blue),
                Print("\n▼ "),
                ResetColor
            )
            .unwrap();
        }

        let time_taken = format!("{:.2?}", elapsed);
        execute!(stdout, Print(format!("| {} | ", time_taken))).unwrap();

        if result.is_err() {
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                Print("An error occurred during tests."),
                ResetColor
            )
            .unwrap();
        } else {
            execute!(stdout, Print("All tests ran successfully.")).unwrap();
        }
    }

    /// Sample test function (can be replaced with user-defined tests).
    pub fn sample_test() {
        let input = crate::i::userinput("What is your name?");
        println!("{}", input);
    }

    /// Runs the default tests block.
    pub fn run() {
        run_tests(|| {
            // Place your tests inside this closure.
            // For example, call your sample test:
            sample_test();

            // You can add more tests here.
            // If any test panics, it will be caught and a red indicator will be shown.
        });
    }
}

#[macro_export]
macro_rules! tests {
    ($($body:tt)*) => {
        $crate::cls("y");
        $crate::testing::run_tests(|| {
            $($body)*
        })
    }
}
