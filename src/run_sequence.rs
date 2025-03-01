use super::*;
use crate::command::*;
use crate::command_executor::*;

pub fn run_sequence(commands: &[Command], executor: &dyn CommandExecutor) {
    info!("Starting sequence execution...");

    let mut pos = 0;
    while pos < commands.len() {
        trace!(
            "Processing command at position {}: {:?}",
            pos,
            commands[pos]
        );

        match &commands[pos].cmd {
            CommandType::CharAndPressType(c) => {
                trace!("Executing char command: {:?}", c);
                if let Err(e) = executor.execute_command_char(c) {
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
