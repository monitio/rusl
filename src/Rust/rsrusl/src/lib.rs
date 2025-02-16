use std::{io::self, process::Command};
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};

/// The current rusl version.
pub const RUSLVER: &str = "0.1.2\n";

/// Prints the current rusl version.
pub fn v() {
    println!("rusl - {}", RUSLVER);
}

/// Prints "Hello, World!"
pub fn hw() {
    println!("Hello, World!");
}

pub fn cls(printversion: &str) {
    // Clear the console
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }

    // Validate input and act accordingly
    match printversion.trim().to_lowercase().as_str() {
        "yes" | "y" => crate::v(),
        "no" | "n" => {}
        _ => panic!("Invalid argument for rusl::cls(printversion). Use 'yes' or 'no'."),
    }
}

/// Logs a message with a colored "🞂" symbol based on status type.
pub fn log(status: &str, message: &str) {
    let mut stdout = io::stdout();

    // Define colors for each status
    let status_color = match status {
        "tip" => Color::Magenta,
        "good" => Color::Green,
        "warning" => Color::DarkYellow, // Yellowish/Orange
        "error" => Color::Red,
        "userhelp" => Color::Cyan, // Light Blue
        _ => Color::White, // Default color for unknown statuses
    };

    execute!(
        stdout,
        SetForegroundColor(status_color),
        Print("▼ "),
        ResetColor,
        Print(message),
    ).unwrap();
}

/// Greets a given name in a random language.
///
/// # Arguments
///
/// * `name` - The name of the person to greet.
pub fn g(name: &str) {
    let greetlanguages = [
        "Hello", "Hi", "Sup", "Howdy", "Greetings", "Namaste",
        "Bonjour", "Salut", "Hola", "Ciao", "Hallo", "Hej", "Hei",
        "Merhaba", "привет", "Привіт", "Γεια σας"
    ];
    // Use fastrand to generate a random index.
    let idx = fastrand::usize(0..greetlanguages.len());
    println!("{}, {}!", greetlanguages[idx], name);
}

pub mod i {
    use std::io::{self, Write};
    use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};
        
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
        let formattedprompt = format!("{} ", prompt);
    
        execute!(
            stdout,
            SetForegroundColor(statuscolor),
            Print(indicator),
            ResetColor,
            Print(formattedprompt),
        ).unwrap();
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut useroutput = String::new();
        io::stdin()
            .read_line(&mut useroutput)
            .expect("Failed to read input");
    
        useroutput.trim().to_string()
    }
}

pub mod testing {
    use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};
    use std::io;
    use std::time::Instant;
    
    /// This function will be used to run the tests and track the time
    pub fn run_tests<F>(test_block: F)
    where
        F: FnOnce(),
    {
        let start_time = Instant::now();
        
        // Run the test block
        test_block();
        
        let elapsed = start_time.elapsed();
        
        // Output the test results
        let mut stdout = io::stdout();
        
        // Color for the test result
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("▼ "),
            ResetColor,
        ).unwrap();
        
        let time_taken = format!("{:?}", elapsed);
        execute!(
            stdout,
            Print(format!("| {} | All tests ran successfully.", time_taken)),
        ).unwrap();
    }
    
    /// Sample test function (can be replaced with user-defined tests)
    pub fn sample_test() {
        let input = crate::i::userinput("What is your name?");
        println!("{}", input);
    }
    
    // To execute the tests block
    pub fn run() {
        run_tests(|| {
            // User can define any tests here
            sample_test();
        });
    }
}
