use std::{thread, time};
use winapi::um::winuser::GetAsyncKeyState;

fn main() {
    println!("Press keys to see their virtual key codes.");
    println!("Press ctrl+c to exit.");
    println!("Write down the hexa code.");
    println!("Find constant name in https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes");
    println!();
  

    loop {
        // Scan all VK codes
        for vk in 0x01..=0xFE {
            let state = unsafe { GetAsyncKeyState(vk) };
            if state==-32768{
                println!("Virtual Key Code: 0x{:X}", vk);
            }
        }
        thread::sleep(time::Duration::from_millis(100)); // Prevent high CPU usage
    }
}
