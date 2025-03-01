#[derive(Debug)]
pub enum InstructionType {
    Goto(usize),
    Noop,
    Exit,
    SleepMs(u64),
    SleepUs(u64),
    SleepS(u64),
    Log(String),
}

/// VK= Virtual Key
pub type VK = u16;

#[derive(Debug)]
pub struct CharAndPressType {
    pub vk: VK,
    pub press_type: PressType,
}

#[derive(Debug)]
pub enum CommandType {
    CharAndPressType(CharAndPressType),
    Instruction(InstructionType),
}

#[derive(Debug)]
pub enum PressType {
    Down,
    Up,
    Pressed,
}

#[derive(Debug)]
pub struct Command {
    pub cmd: CommandType,
}
