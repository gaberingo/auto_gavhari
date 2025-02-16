use std::io::{stdin, stdout};

// crossterm digunakan untuk pemberian warna pada terminal
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print question dengan spesifik warna
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read input user
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return
    return user_response.trim().to_string();
}
