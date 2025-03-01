mod command;
mod command_executor;
mod parse_command;
mod run_sequence;

use command_executor::*;
use parse_command::sequence_decoder;
use run_sequence::run_sequence;
use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tracing::*;

#[cfg(test)]
mod parse_command_test;
#[cfg(test)]
mod run_sequence_test;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let sequence = env::var("SEQUENCE").unwrap_or_else(|_| "".to_string());
    let delimiter = env::var("DELIMITER").unwrap_or_else(|_| "#".to_string());

    let executor: &dyn CommandExecutor;
    #[cfg(not(windows))]
    {
        println!("This is not a Windows platform, key press mocked");
        executor = &PrintExecutor
    }
    #[cfg(windows)]
    {
        executor = &WindowsPressExecutor
    }

    match sequence_decoder(delimiter, sequence) {
        Ok(cmds) => run_sequence(&cmds, executor),
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
    println!(
        "  DELIMITER - single character delimiter for commands (default: #, excluded: _ ^ space)"
    );
    println!("Commands to input a character:");
    println!("  *          - Any single character is treated as input (press+release)");
    println!("  ^*         - Press this character");
    println!("  _*         - Release this character");
    println!("  ASCII n    - inputs the ASCII character with decimal code n (press+release)");
    println!("  ^ASCII n   - press the ASCII character with decimal code n");
    println!("  _ASCII n   - release the ASCII character with decimal code n");
    println!("Commands for instructions:");
    println!("  REM *      - this is a comment, ignored");
    println!("  GOTO n     - jumps to position n (starts at 0)");
    println!("  EXIT       - stops execution");
    println!("  SLEEPMS n  - sleeps n milliseconds");
    println!("  SLEEPUS n  - sleeps n microseconds");
    println!("  SLEEPS n   - sleeps n seconds");
    println!("  LOG msg    - logs a message");
    //FIXME example
}
