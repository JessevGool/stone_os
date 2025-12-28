use crate::print;
use crate::println;
use alloc::string::String;
use alloc::vec::Vec;
use pc_keyboard::{DecodedKey, KeyCode};

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
            DecodedKey::Unicode(character) => {
                if character == '\n' {
                    println!();
                    self.execute_command();
                    self.buffer.clear();
                    print!("$ ");
                } else if character == '\u{8}' {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        print!("\u{8} \u{8}");
                    }
                } else {
                    self.buffer.push(character);
                    print!("{}", character);
                }
            }
            DecodedKey::RawKey(KeyCode::NumpadEnter) => {
                println!();
                self.execute_command();
                self.buffer.clear();
                print!("$ ");
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