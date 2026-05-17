use std::io::{Write, stdin, stdout};
mod functions;
use functions::cd::cd_process;
use functions::clear::clear_process;
use functions::command::command_functions;
use functions::echo::echoing;
use functions::pwd::pwd_process;

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => cd_process(&mut args.peekable(), &mut previous_command),
                "exit" => return,
                "pwd" => pwd_process(),
                "echo" => echoing(&mut args.peekable()),
                "clear" => clear_process(),
                command => command_functions(
                    command,
                    &mut args.peekable(),
                    &mut previous_command,
                    &mut commands,
                ),
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            let _ = final_command.wait();
        }
    }
}
