use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::sync::Mutex;

    // A mock executor for testing purposes
    struct MockExecutor {
        called_char: Arc<Mutex<Vec<char>>>, // Keep track of characters executed
    }

    impl MockExecutor {
        fn new() -> Self {
            MockExecutor {
                called_char: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl CommandExecutor for MockExecutor {
        fn execute_command_char(&self, c: char) -> Result<(), Box<dyn std::error::Error>> {
            let mut called_char = self.called_char.lock().unwrap();
            called_char.push(c);
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_run_sequence() {
            let executor = MockExecutor::new();
            let commands = vec![
                Command {
                    cmd: CommandType::Char('a'),
                },
                Command {
                    cmd: CommandType::Instruction(InstructionType::SleepUs(1)),
                },
                Command {
                    cmd: CommandType::Char('b'),
                },
                Command {
                    cmd: CommandType::Instruction(InstructionType::Exit),
                },
            ];

            // Run the sequence
            run_sequence(&commands, &executor);

            // Check if 'a' and 'b' were executed
            let called_char = executor.called_char.lock().unwrap();
            assert_eq!(*called_char, vec!['a', 'b']);
        }
    }
}
