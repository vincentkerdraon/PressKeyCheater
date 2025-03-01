//https://docs.rs/crate/winapi/latest
#[cfg(windows)]
extern crate winapi;
use super::*;
use crate::command::*;

#[cfg(windows)]
use winapi::um::winuser::{keybd_event, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VK_NUMPAD0};

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
    fn down(&self, key: u8) {
        unsafe {
            keybd_event(key, 0, KEYEVENTF_UNICODE, 0);
        }
    }
    fn up(&self, key: u8) {
        unsafe {
            keybd_event(key, 0, KEYEVENTF_KEYUP, 0);
        }
    }

    fn execute_command_char(&self, c: &CharAndPressType) -> Result<(), Box<dyn Error>> {
        let vk = VK_NUMPAD0 as u8; //FIXME from c
        match press_type {
            PressType::Down => self::down(),
            PressType::Up => self::up(),
            PressType::Pressed => {
                self::down();
                self::up()
            }
        }
        Ok(())
    }
}
