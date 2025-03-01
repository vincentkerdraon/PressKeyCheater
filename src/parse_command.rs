use super::*;

pub fn sequence_decoder(
    delimiter: String,
    sequence: String,
) -> Result<Vec<Command>, Box<dyn Error>> {
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
    } else if delimiter.chars().count() != 1 {
        Err("DELIMITER must be a single character.".into())
    } else {
        Ok(delimiter.chars().next().unwrap())
    }
}

pub fn parse_command(cmd: &str) -> Result<CommandType, Box<dyn Error>> {
    if cmd.is_empty() {
        Ok(CommandType::Instruction(InstructionType::Noop))
    } else if cmd.len() == 1 {
        // If it's a single character, return it as a char command
        Ok(CommandType::Char(cmd.chars().next().unwrap()))
    } else {
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
                .map(|v| CommandType::Char(v as char))
                .ok_or_else(|| "ASCII requires a valid number".into()),

            _ => Err(format!("Unknown command {}", parts[0]).into()), // Return an error for unknown commands
        }
    }
}
