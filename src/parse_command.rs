use super::*;
use crate::command::*;


pub fn sequence_decoder(
    delimiter: String,
    sequence: String,
) -> Result<Vec<Command>, Box<dyn Error>> {
    if sequence.is_empty() {
        return Err("SEQUENCE cannot be empty".into());
    }
    if sequence.starts_with(&delimiter) {
        return Err("SEQUENCE cannot start with a delimiter".into());
    }
    if sequence.ends_with(&delimiter) {
        return Err("SEQUENCE cannot end with a delimiter".into());
    }

    let d = get_delimiter(&delimiter)?;
    let mut result = Vec::new();
    for (i, cmd) in sequence.split(d).enumerate() {
        match parse_command(cmd) {
            Ok(cmd_type) => result.push(Command { cmd: cmd_type }),
            Err(e) => return Err(format!("Error parsing command at position {}: {}", i, e).into()),
        }
    }

    Ok(result)
}

fn get_delimiter(delimiter: &str) -> Result<char, Box<dyn Error>> {
    if delimiter == "\\n" {
        Ok('\n')
    } else if delimiter == " " {
        Err("DELIMITER space is not allowed.".into())
    } else if delimiter == "^" {
        Err("DELIMITER ^ is not allowed.".into())
    } else if delimiter == "_" {
        Err("DELIMITER _ is not allowed.".into())
    } else if delimiter.chars().count() != 1 {
        Err("DELIMITER must be a single character.".into())
    } else {
        Ok(delimiter.chars().next().unwrap())
    }
}

pub fn parse_command(cmd: &str) -> Result<CommandType, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(CommandType::Instruction(InstructionType::Noop));
    }

    if cmd.len() == 1 {
        // If it's a single character, return it as a char command
        return Ok(CommandType::CharAndPressType(CharAndPressType {
            c: cmd.chars().next().unwrap(),
            press_type: PressType::Pressed,
        }));
    }

    if cmd.len() == 2 {
        if cmd.starts_with("^") {
            if let Some(c) = cmd.chars().nth(1) {
                return Ok(CommandType::CharAndPressType(CharAndPressType {
                    c: c,
                    press_type: PressType::Up,
                }));
            } else {
                return Err(format!("Invalid line {}", cmd).into());
            }
        }
        if cmd.starts_with("_") {
            if let Some(c) = cmd.chars().nth(1) {
                return Ok(CommandType::CharAndPressType(CharAndPressType {
                    c: c,
                    press_type: PressType::Down,
                }));
            } else {
                return Err(format!("Invalid line {}", cmd).into());
            }
        }
        return Err(format!("Invalid line {}", cmd).into());
    }

    let parts: Vec<&str> = cmd.split_whitespace().collect();
    match parts[0] {
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
            .map(|v| {
                CommandType::CharAndPressType(CharAndPressType {
                    c: v as char,
                    press_type: PressType::Pressed,
                })
            })
            .ok_or_else(|| "ASCII requires a valid number".into()),
        "_ASCII" => parts
            .get(1)
            .and_then(|v| v.parse::<u8>().ok())
            .map(|v| {
                CommandType::CharAndPressType(CharAndPressType {
                    c: v as char,
                    press_type: PressType::Down,
                })
            })
            .ok_or_else(|| "_ASCII requires a valid number".into()),
        "^ASCII" => parts
            .get(1)
            .and_then(|v| v.parse::<u8>().ok())
            .map(|v| {
                CommandType::CharAndPressType(CharAndPressType {
                    c: v as char,
                    press_type: PressType::Up,
                })
            })
            .ok_or_else(|| "^ASCII requires a valid number".into()),

        _ => Err(format!("Unknown command {}", parts[0]).into()), // Return an error for unknown commands
    }
}
