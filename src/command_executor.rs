use super::*;
use crate::command::*;

//https://docs.rs/crate/winapi/latest
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use winapi::um::winuser::KEYEVENTF_KEYUP;

pub trait CommandExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>>;
}

#[cfg(windows)]
pub struct WindowsPressExecutor;

#[cfg(windows)]
impl WindowsPressExecutor {
    fn press_key(key: u16, flags: u32) {
        use winapi::um::winuser::{INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT};

        let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };
        unsafe {
            *input_u.ki_mut() = KEYBDINPUT {
                wVk: key,
                dwExtraInfo: 0,
                wScan: 0,
                time: 0,
                dwFlags: flags,
            }
        }

        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: input_u,
        };
        let ipsize = std::mem::size_of::<INPUT>() as i32;
        unsafe {
            SendInput(1, &mut input, ipsize);
        };
    }
}

#[cfg(windows)]
impl CommandExecutor for WindowsPressExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>> {
        match c.press_type {
            PressType::Down => {
                command_executor::WindowsPressExecutor::press_key(c.vk, 0);
            }
            PressType::Up => {
                command_executor::WindowsPressExecutor::press_key(c.vk, KEYEVENTF_KEYUP);
            }
            PressType::Pressed => {
                command_executor::WindowsPressExecutor::press_key(c.vk, 0);
                command_executor::WindowsPressExecutor::press_key(c.vk, KEYEVENTF_KEYUP);
            }
        }
        Ok(())
    }
}

pub struct PrintExecutor;

impl CommandExecutor for PrintExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>> {
        match c.press_type {
            PressType::Down => {
                println!("KEYDOWN {}", c.vk);
            }
            PressType::Up => {
                println!("KEYUP {}", c.vk);
            }
            PressType::Pressed => {
                println!("KEYPRESSED {}", c.vk);
            }
        }
        Ok(())
    }
}
