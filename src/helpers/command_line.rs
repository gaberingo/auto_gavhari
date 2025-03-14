use std::io::{stdin, stdout};

// crossterm digunakan untuk pemberian warna pada terminal
use crossterm::{
    style::{Color, Colors, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print Color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement in a specific Color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected Color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_print_agent_msg() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "testing some output");
    }
}
