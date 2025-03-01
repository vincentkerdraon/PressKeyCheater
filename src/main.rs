mod parse_command;
mod run_sequence;

use parse_command::sequence_decoder;
use run_sequence::run_sequence;
use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, error, info, trace};

#[cfg(test)]
mod parse_command_test;
mod run_sequence_test;

#[derive(Debug)]
enum InstructionType {
    Goto(usize),
    Noop,
    Exit,
    SleepMs(u64),
    SleepUs(u64),
    SleepS(u64),
    Log(String),
}

#[derive(Debug)]
enum CommandType {
    Char(char),
    Instruction(InstructionType),
}

#[derive(Debug)]
struct Command {
    cmd: CommandType,
}

trait CommandExecutor {
    fn execute_command_char(&self, c: char) -> Result<(), Box<dyn Error>>;
}

struct PrintExecutor;

impl CommandExecutor for PrintExecutor {
    fn execute_command_char(&self, c: char) -> Result<(), Box<dyn Error>> {
        println!("{}", c);
        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let sequence = env::var("SEQUENCE").unwrap_or_else(|_| "".to_string());
    let delimiter = env::var("DELIMITER").unwrap_or_else(|_| "#".to_string());

    if sequence.is_empty() {
        help(Some("SEQUENCE cannot be empty".into()));
        return;
    }

    if sequence.ends_with(&delimiter) {
        help(Some("SEQUENCE cannot end with a delimiter".into()));
        return;
    }

    match sequence_decoder(delimiter, sequence) {
        Ok(cmds) => run_sequence(&cmds, &PrintExecutor),
        Err(e) => help(Some(e)),
    }
}

fn help(err: Option<Box<dyn Error>>) {
    if let Some(e) = err {
        println!("\nError: {}\n", e);
    }
    println!("Usage: PressKeyCheater");
    println!("Set environment variables:");
    println!("  SEQUENCE - the sequence of commands");
    println!("  DELIMITER - single character delimiter for commands");
    println!("Commands:");
    println!("  GOTO n     - jumps to position n (starts at 0)");
    println!("  EXIT       - stops execution");
    println!("  SLEEPMS n  - sleeps n milliseconds");
    println!("  SLEEPUS n  - sleeps n microseconds");
    println!("  SLEEPS n   - sleeps n seconds");
    println!("  LOG msg    - logs a message");
    println!("  ASCII n    - inputs the ASCII character with decimal code n");
    println!("  Any single character is treated as input");
}
