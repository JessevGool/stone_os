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
