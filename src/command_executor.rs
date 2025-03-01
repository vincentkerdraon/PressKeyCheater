use super::*;
use crate::command::*;

//https://docs.rs/crate/winapi/latest
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
use winapi::um::winuser::{
    SendInput, VkKeyScanW, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
};

pub trait CommandExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>>;
}

pub struct PrintExecutor;

impl CommandExecutor for PrintExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>> {
        match c.press_type {
            PressType::Down => {
                println!("KEYDOWN {}", c.c);
            }
            PressType::Up => {
                println!("KEYUP {}", c.c);
            }
            PressType::Pressed => {
                println!("KEYPRESSED {}", c.c);
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
pub struct WindowsPressExecutor;

#[cfg(windows)]
impl CommandExecutor for WindowsPressExecutor {
    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>> {
        unsafe {
            let vk = VkKeyScanW(c.c as u16) as u16; // Convert char to virtual key

            if vk == 0xFFFF {
                return Err(format!("Unsupported character: {}", c.c).into());
            }

            let mut input = INPUT {
                type_: INPUT_KEYBOARD,
                u: std::mem::zeroed(),
            };

            let ki = &mut input.u.ki;
            ki.wVk = vk; // Set virtual key

            match c.press_type {
                PressType::Down => {
                    ki.dwFlags = 0;
                }
                PressType::Up => {
                    ki.dwFlags = KEYEVENTF_KEYUP;
                }
                PressType::Pressed => {
                    ki.dwFlags = 0;
                    SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
                    ki.dwFlags = KEYEVENTF_KEYUP;
                }
            }

            SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
        }
        Ok(())
    }
}
