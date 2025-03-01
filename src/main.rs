use std::env;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, error, info, trace};

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
    tracing_subscriber::fmt::init();

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

fn sequence_decoder(delimiter: String, sequence: String) -> Result<Vec<Command>, Box<dyn Error>> {
    let d = if delimiter == "\\n" {
        '\n'
    } else if delimiter == " " {
        return Err("DELIMITER space is not possible.".into());
    } else if delimiter.chars().count() != 1 {
        return Err("DELIMITER is mandatory. It must be one character".into());
    } else {
        delimiter.chars().next().unwrap()
    };

    let mut result = Vec::new();
    for (i, cmd) in sequence.split(d).enumerate() {
        match parse_command(cmd) {
            Ok(cmd_type) => result.push(Command { cmd: cmd_type }),
            Err(e) => return Err(format!("Error parsing command at position {}: {}", i, e).into()),
        }
    }

    Ok(result)
}
fn parse_command(cmd: &str) -> Result<CommandType, Box<dyn Error>> {
    if cmd.len() == 1 {
        // If it's a single character, return it as a char command
        Ok(CommandType::Char(cmd.chars().next().unwrap()))
    } else {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts[0] {
            "" => Ok(CommandType::Instruction(InstructionType::Noop)),
            "REM" => Ok(CommandType::Instruction(InstructionType::Noop)),
            "GOTO" => parts
                .get(1)
                .and_then(|v| v.parse::<usize>().ok())
                .map(InstructionType::Goto)
                .map(CommandType::Instruction)
                .ok_or_else(|| "GOTO requires a valid position".into()),

            "EXIT" => Ok(CommandType::Instruction(InstructionType::Exit)),

            "SLEEPMS" => parts
                .get(1)
                .and_then(|v| v.parse::<u64>().ok())
                .map(InstructionType::SleepMs)
                .map(CommandType::Instruction)
                .ok_or_else(|| "SLEEPMS requires a valid number".into()),

            "SLEEPUS" => parts
                .get(1)
                .and_then(|v| v.parse::<u64>().ok())
                .map(InstructionType::SleepUs)
                .map(CommandType::Instruction)
                .ok_or_else(|| "SLEEPUS requires a valid number".into()),

            "SLEEPS" => parts
                .get(1)
                .and_then(|v| v.parse::<u64>().ok())
                .map(InstructionType::SleepS)
                .map(CommandType::Instruction)
                .ok_or_else(|| "SLEEPS requires a valid number".into()),

            "LOG" => {
                let log_message = parts[1..].join(" ");
                Ok(CommandType::Instruction(InstructionType::Log(log_message)))
            }

            "ASCII" => parts
                .get(1)
                .and_then(|v| v.parse::<u8>().ok())
                .map(|v| CommandType::Char(v as char))
                .ok_or_else(|| "ASCII requires a valid number".into()),

            _ => Err(format!("Unknown command {}", parts[0]).into()), // Return an error for unknown commands
        }
    }
}

fn run_sequence(commands: &[Command], executor: &dyn CommandExecutor) {
    info!("Starting sequence execution...");

    let mut pos = 0;
    while pos < commands.len() {
        trace!(
            "Processing command at position {}: {:?}",
            pos,
            commands[pos]
        );

        match &commands[pos].cmd {
            CommandType::Char(c) => {
                trace!("Executing char command: {}", c);
                if let Err(e) = executor.execute_command_char(*c) {
                    error!("Error executing char command: {}", e);
                }
            }
            CommandType::Instruction(instr) => match instr {
                InstructionType::Noop => {}
                InstructionType::Goto(v) => {
                    trace!("GOTO position {}", *v);
                    if *v < commands.len() {
                        pos = *v;
                        continue;
                    }
                }
                InstructionType::Exit => {
                    info!("Exiting sequence execution.");
                    return;
                }
                InstructionType::SleepMs(ms) => {
                    trace!("Sleeping for {} milliseconds", ms);
                    sleep(Duration::from_millis(*ms));
                }
                InstructionType::SleepUs(us) => {
                    trace!("Sleeping for {} microseconds", us);
                    sleep(Duration::from_micros(*us));
                }
                InstructionType::SleepS(s) => {
                    trace!("Sleeping for {} seconds", s);
                    sleep(Duration::from_secs(*s));
                }
                InstructionType::Log(msg) => {
                    debug!("LOG: {}", msg);
                }
            },
        }
        pos += 1;
    }

    info!("Sequence execution completed.");
}

fn help(err: Option<Box<dyn Error>>) {
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
    if let Some(e) = err {
        println!("Error: {}", e);
    }
}
