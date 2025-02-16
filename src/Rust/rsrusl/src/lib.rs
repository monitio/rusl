// src/lib.rs

pub mod m {
    /// Prints "Hello, World!"
    pub fn hw() {
        println!("Hello, World!");
    }
    
    /// Greets a given name in a random language.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the person to greet.
    pub fn g(name: &str) {
        let greet_languages = [
            "Hello", "Hi", "Sup", "Howdy", "Greetings", "Namaste",
            "Bonjour", "Salut", "Hola", "Ciao", "Hallo", "Hej", "Hei",
            "Merhaba", "привет", "Привіт", "Γεια σας"
        ];
        // Use fastrand to generate a random index.
        let idx = fastrand::usize(0..greet_languages.len());
        println!("{}, {}!", greet_languages[idx], name);
    }
    
    pub mod i {
        use std::io::{self, Write};
        
        /// Prompts the user for input and returns the entered string.
        ///
        /// # Arguments
        ///
        /// * `prompt` - The prompt to display to the user.
        pub fn userinput(prompt: &str) -> String {
            print!("{}", prompt);
            io::stdout().flush().expect("Failed to flush stdout");
            
            let mut user_output = String::new();
            io::stdin()
                .read_line(&mut user_output)
                .expect("Failed to read input");
            
            user_output.trim().to_string()
        }
    }
}

// Re-export functions for direct use as rusl::hw(), rusl::g(), etc.
pub use m::hw;
pub use m::g;
pub use m::i::userinput;
