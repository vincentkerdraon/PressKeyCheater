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

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let sequence = env::var("SEQUENCE").unwrap_or_else(|_| "".to_string());
    let delimiter = env::var("DELIMITER").unwrap_or_else(|_| "#".to_string());

    #[cfg(not(windows))]
    {
        let executor: &dyn CommandExecutor = &PrintExecutor;
        warn!("This is not a Windows platform, key press mocked !!");
    }

    #[cfg(windows)]
    let executor: &dyn CommandExecutor = &WindowsPressExecutor;

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
    println!("Once started, this program will execute the sequence. Even if it means destroying your computer...");

    println!("Set environment variables:");
    println!("  SEQUENCE - the sequence of commands");
    println!(
        "  DELIMITER - single character delimiter for commands (default: #, excluded: _ ^ space)"
    );
    println!("Commands to input a character:");
    println!("  VK *       - the code for a key like NUMPAD0 or SHIFT (press+release)");
    println!("  ^VK *      - Release this key (release only)");
    println!("  _VK *      - Press this key (no release)");

    println!("Commands for instructions:");
    println!("  REM *      - this is a comment, ignored");
    println!("  GOTO n     - jumps to position n (starts at 0)");
    println!("  EXIT       - stops execution");
    println!("  SLEEPMS n  - sleeps n milliseconds");
    println!("  SLEEPUS n  - sleeps n microseconds");
    println!("  SLEEPS n   - sleeps n seconds");
    println!("  LOG msg    - logs a message");

    println!("Examples:");
    println!("  SEQUENCE=\"VK H.VK E.VK L.VK L.VK O\" DELIMITER=\".\"");
    println!("     (default delimiter: #)");
    println!("  SEQUENCE=\"VK H#VK E#VK L#VK L#VK O#VK SPACE#SLEEPS 1#GOTO 0\"");
    println!("     (If you use GOTO, always put a sleep, because it will be fast as fuck.)");
    println!("  SEQUENCE=\"VK A#_VK SHIFT#VK A#^VK SHIFT#VK A\"");
    println!("Suggested call: create a .bat each time, declaring env var and calling this program. Sequence delimiter can be a line break");
    println!("");
}
