use super::*;

#[cfg(test)]
mod tests {
    use crate::{command::*, parse_command::parse_command};

    use super::*;

    #[test]
    fn test_parse_command_vk() {
        let cmd = "VK A";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::CharAndPressType(CharAndPressType {
                vk,
                press_type: PressType::Pressed,
            })) if vk == 0x41 => (),
            _ => panic!(
                "Expected Ok(CommandType::CharAndPressType('a', Pressed)), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_goto_valid() {
        let cmd = "GOTO 10";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::Goto(10))) => (),
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::Goto(10))), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_goto_invalid() {
        let cmd = "GOTO";
        let result = parse_command(cmd);
        match result {
            Err(e) => assert_eq!(e.to_string(), "GOTO requires a valid position"),
            _ => panic!(
                "Expected Err with message 'GOTO requires a valid position', got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_exit() {
        let cmd = "EXIT";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::Exit)) => (),
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::Exit)), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_sleepms_valid() {
        let cmd = "SLEEPMS 500";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::SleepMs(500))) => (),
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::SleepMs(500))), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_sleepms_invalid() {
        let cmd = "SLEEPMS";
        let result = parse_command(cmd);
        match result {
            Err(e) => assert_eq!(e.to_string(), "SLEEPMS requires a valid number"),
            _ => panic!(
                "Expected Err with message 'SLEEPMS requires a valid number', got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_log() {
        let cmd = "LOG Hello, World!";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::Log(msg))) => {
                assert_eq!(msg, "Hello, World!");
            }
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::Log(..))), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_rem() {
        let cmd = "REM This is a comment";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::Noop)) => (),
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::Noop)), got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_unknown() {
        let cmd = "UNKNOWN_CMD";
        let result = parse_command(cmd);
        match result {
            Err(e) => assert_eq!(e.to_string(), "Unknown command UNKNOWN_CMD"),
            _ => panic!(
                "Expected Err with message 'Unknown command UNKNOWN_CMD', got {:?}",
                result
            ),
        }
    }

    #[test]
    fn test_parse_command_empty() {
        let cmd = "";
        let result = parse_command(cmd);
        match result {
            Ok(CommandType::Instruction(InstructionType::Noop)) => {}
            _ => panic!(
                "Expected Ok(CommandType::Instruction(InstructionType::Noop)), got {:?}",
                result
            ),
        }
    }
}
