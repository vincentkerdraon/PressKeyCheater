use super::*;
use crate::command::*;

fn vk_name_to_code(key: &str) -> Option<u16> {
    match key {
        "LBUTTON" => Some(0x01),
        "RBUTTON" => Some(0x02),
        "CANCEL" => Some(0x03),
        "MBUTTON" => Some(0x04),
        "XBUTTON1" => Some(0x05),
        "XBUTTON2" => Some(0x06),
        "BACK" => Some(0x08),
        "TAB" => Some(0x09),
        "CLEAR" => Some(0x0C),
        "RETURN" => Some(0x0D),
        "SHIFT" => Some(0x10),
        "CONTROL" => Some(0x11),
        "MENU" => Some(0x12), // ALT key
        "PAUSE" => Some(0x13),
        "CAPITAL" => Some(0x14), // CAPS LOCK
        "ESCAPE" => Some(0x1B),
        "SPACE" => Some(0x20),
        "PRIOR" => Some(0x21), // PAGE UP
        "NEXT" => Some(0x22),  // PAGE DOWN
        "END" => Some(0x23),
        "HOME" => Some(0x24),
        "LEFT" => Some(0x25),
        "UP" => Some(0x26),
        "RIGHT" => Some(0x27),
        "DOWN" => Some(0x28),
        "SELECT" => Some(0x29),
        "PRINT" => Some(0x2A),
        "EXECUTE" => Some(0x2B),
        "SNAPSHOT" => Some(0x2C), // PRINT SCREEN
        "INSERT" => Some(0x2D),
        "DELETE" => Some(0x2E),
        "HELP" => Some(0x2F),

        // Digits 0-9
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),

        // Letters A-Z
        "A" => Some(0x41),
        "B" => Some(0x42),
        "C" => Some(0x43),
        "D" => Some(0x44),
        "E" => Some(0x45),
        "F" => Some(0x46),
        "G" => Some(0x47),
        "H" => Some(0x48),
        "I" => Some(0x49),
        "J" => Some(0x4A),
        "K" => Some(0x4B),
        "L" => Some(0x4C),
        "M" => Some(0x4D),
        "N" => Some(0x4E),
        "O" => Some(0x4F),
        "P" => Some(0x50),
        "Q" => Some(0x51),
        "R" => Some(0x52),
        "S" => Some(0x53),
        "T" => Some(0x54),
        "U" => Some(0x55),
        "V" => Some(0x56),
        "W" => Some(0x57),
        "X" => Some(0x58),
        "Y" => Some(0x59),
        "Z" => Some(0x5A),

        // Function Keys
        "F1" => Some(0x70),
        "F2" => Some(0x71),
        "F3" => Some(0x72),
        "F4" => Some(0x73),
        "F5" => Some(0x74),
        "F6" => Some(0x75),
        "F7" => Some(0x76),
        "F8" => Some(0x77),
        "F9" => Some(0x78),
        "F10" => Some(0x79),
        "F11" => Some(0x7A),
        "F12" => Some(0x7B),

        // Numpad keys
        "NUMPAD0" => Some(0x60),
        "NUMPAD1" => Some(0x61),
        "NUMPAD2" => Some(0x62),
        "NUMPAD3" => Some(0x63),
        "NUMPAD4" => Some(0x64),
        "NUMPAD5" => Some(0x65),
        "NUMPAD6" => Some(0x66),
        "NUMPAD7" => Some(0x67),
        "NUMPAD8" => Some(0x68),
        "NUMPAD9" => Some(0x69),
        "MULTIPLY" => Some(0x6A),
        "ADD" => Some(0x6B),
        "SEPARATOR" => Some(0x6C),
        "SUBTRACT" => Some(0x6D),
        "DECIMAL" => Some(0x6E),
        "DIVIDE" => Some(0x6F),

        // Modifier keys
        "LSHIFT" => Some(0xA0),
        "RSHIFT" => Some(0xA1),
        "LCONTROL" => Some(0xA2),
        "RCONTROL" => Some(0xA3),
        "LMENU" => Some(0xA4), // Left ALT
        "RMENU" => Some(0xA5), // Right ALT

        // Windows keys
        "LWIN" => Some(0x5B),
        "RWIN" => Some(0x5C),
        "APPS" => Some(0x5D), // Application key

        // OEM Keys
        "OEM_1" => Some(0xBA),      // US standard keyboard `;:`
        "OEM_PLUS" => Some(0xBB),   // `+`
        "OEM_COMMA" => Some(0xBC),  // `,`
        "OEM_MINUS" => Some(0xBD),  // `-`
        "OEM_PERIOD" => Some(0xBE), // `.`
        "OEM_2" => Some(0xBF),      // US standard keyboard `/ ?`
        "OEM_3" => Some(0xC0),      // US standard keyboard `` ` ~ ``
        "OEM_4" => Some(0xDB),      // `[ {`
        "OEM_5" => Some(0xDC),      // `\ |`
        "OEM_6" => Some(0xDD),      // `] }`
        "OEM_7" => Some(0xDE),      // `' "`
        "OEM_8" => Some(0xDF),

        _ => None,
    }
}

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
    if delimiter.is_empty() {
        Err("DELIMITER not defined.".into())
    } else if delimiter == "\\n" {
        Ok('\n')
    } else if delimiter == " " {
        Err("DELIMITER space is not allowed.".into())
    } else if delimiter == "^" {
        Err("DELIMITER ^ is not allowed.".into())
    } else if delimiter == "_" {
        Err("DELIMITER _ is not allowed.".into())
    } else if delimiter.chars().count() != 1 {
        Err("DELIMITER must be a single character or \\n.".into())
    } else {
        Ok(delimiter.chars().next().unwrap())
    }
}

pub fn parse_command(cmd: &str) -> Result<CommandType, Box<dyn Error>> {
    if cmd.is_empty() {
        return Ok(CommandType::Instruction(InstructionType::Noop));
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

        "VK" | "_VK" | "^VK" => {
            if let Some(key_name) = parts.get(1) {
                if let Some(vk_code) = vk_name_to_code(key_name) {
                    let press_type = match parts[0] {
                        "VK" => PressType::Pressed,
                        "_VK" => PressType::Down,
                        "^VK" => PressType::Up,
                        _ => unreachable!(),
                    };
                    Ok(CommandType::CharAndPressType(CharAndPressType {
                        vk: vk_code,
                        press_type,
                    }))
                } else {
                    Err(format!("Unknown virtual key: {}", key_name).into())
                }
            } else {
                Err("Virtual Key requires a valid name.".into())
            }
        }

        _ => Err(format!("Unknown command {}", parts[0]).into()), // Return an error for unknown commands
    }
}
