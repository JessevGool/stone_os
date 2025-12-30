use alloc::string::ToString;

use super::Shell;
use crate::println;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub execute: fn(&mut Shell, &[&str]),
}

pub fn commands() -> &'static [Command] {
    &[
        Command {
            name: "help",
            description: "Show this help message",
            execute: cmd_help,
        },
        Command {
            name: "echo",
            description: "Print the provided text",
            execute: cmd_echo,
        },
        Command {
            name: "history",
            description: "Show command history",
            execute: cmd_history,
        },
        Command {
            name: "clear",
            description: "Clear the screen",
            execute: cmd_clear,
        },
        Command {
            name: "shutdown",
            description: "Shut down the system",
            execute: cmd_shutdown,
        },
        Command {
            name: "setValue",
            description: "Set a value in the system state",
            execute: cmd_set_value,
        },
        Command {
            name: "getValue",
            description: "Get a value from the system state",
            execute: cmd_get_value,
        }
    ]
}

fn cmd_help(shell: &mut Shell, _args: &[&str]) {
    println!("Available commands:");
    for cmd in commands() {
        println!("{} - {}", cmd.name, cmd.description);
    }
    let _ = shell; // silence unused warning when help is minimal
}

fn cmd_echo(_shell: &mut Shell, args: &[&str]) {
    println!("{}", args.join(" "));
}

fn cmd_history(shell: &mut Shell, _args: &[&str]) {
    for (i, command) in shell.history.iter().enumerate() {
        println!("{}: {}", i + 1, command);
    }
}

fn cmd_clear(shell: &mut Shell, _args: &[&str]) {
    use crate::vga_buffer::WRITER;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().clear_screen();
    });
}

fn cmd_shutdown(_shell: &mut Shell, _args: &[&str]) {
    crate::println!("Shutting down...");

    if let Err(e) = crate::state::State::current().save() {
        crate::println!("Failed to save state: {:?}", e);
    } else {
        crate::println!("State saved successfully.");
    }
    crate::exit_qemu(crate::QemuExitCode::Success);
}

fn cmd_set_value(_shell: &mut Shell, args: &[&str]) {
    use crate::state::State;
    if args.len() != 2 {
        println!("Usage: setValue <key> <value>");
        return;
    }
    let value = args[1].to_string();
    
    State::update(args[0].to_string(), value.clone());
    println!("Value set to: {}", value);
}

fn cmd_get_value(_shell: &mut Shell, _args: &[&str]) {
    use crate::state::State;
    let state = State::current();
    if _args.len() != 1 {
        println!("Usage: getValue <key>");
        return;
    }
    let key = _args[0];
    match state.get_value(key) {
        Some(value) => println!("Current value for {}: {}", key, value),
        None => println!("No value found for key: {}", key),
    }
}