use crate::print;
use crate::println;
use crate::vga_buffer::WRITER;
use alloc::string::String;
use alloc::vec::Vec;
use pc_keyboard::{DecodedKey, KeyCode};
use x86_64::instructions::interrupts;

pub mod command;

pub struct Shell {
    buffer: String,
    pub(crate) history: Vec<String>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            buffer: String::new(),
            history: Vec::new(),
        }
    }

pub fn handle_key(&mut self, key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => match character {
            '\n' => {
                println!();
                self.execute_command();
                self.buffer.clear();
                print!("$ ");
            }
            '\u{8}' => {
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                    interrupts::without_interrupts(|| {
                        WRITER.lock().backspace();
                    });
                }
            }
            _ => {
                self.buffer.push(character);
                print!("{}", character);
            }
        },
        DecodedKey::RawKey(key_code) => {
            match key_code {
                KeyCode::NumpadEnter | KeyCode::Return => {
                    println!();
                    self.execute_command();
                    self.buffer.clear();
                    print!("$ ");
                }
                KeyCode::Backspace => {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        interrupts::without_interrupts(|| {
                            WRITER.lock().backspace();
                        });
                    }
                }
                KeyCode::Delete => {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        interrupts::without_interrupts(|| {
                            WRITER.lock().backspace();
                        });
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

    fn execute_command(&mut self) {
        let line = core::mem::take(&mut self.buffer);
        let cmd = line.trim();
        if cmd.is_empty() {
            return;
        }
        self.history.push(cmd.into());

        let mut parts = cmd.split_whitespace();
        let name = match parts.next() {
            Some(n) => n,
            None => return,
        };
        let args: Vec<&str> = parts.collect();

        if let Some(entry) = command::commands().iter().find(|c| c.name == name) {
            (entry.execute)(self, &args);
        } else {
            println!("Unknown command: {}", name);
        }
    }
}
