use std::io::{Write, stdin, stdout};
mod functions;
use functions::cat::cat_process;
use functions::cd::cd_process;
use functions::clear::clear_process;
use functions::command::command_functions;
use functions::echo::echoing;
use functions::jobs::JobManager;
use functions::kill::kill_process;
use functions::ls::ls_process;
use functions::mkdir::mkdir_process;
use functions::pwd::pwd_process;
use functions::rm::rm_process;
use functions::rmdir::rmdir_process;
use functions::touch::touch_process;

fn main() {
    let mut job_manager = JobManager::new();

    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = input.trim();
        let is_background = if input.ends_with('&') {
            input = input[..input.len() - 1].trim_end();
            true
        } else {
            false
        };

        // must be peekable so we know when we are on the last command
        let mut commands = input.split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "bg" => job_manager.bg_job(args),
                "cat" => cat_process(args),
                "cd" => cd_process(&mut args.peekable(), &mut previous_command),
                "clear" => clear_process(),
                "echo" => echoing(args),
                "exit" => return,
                "fg" => job_manager.fg_job(args),
                "jobs" => job_manager.list_jobs(),
                "kill" => kill_process(args),
                "ls" => ls_process(args),
                "mkdir" => mkdir_process(args),
                "pwd" => pwd_process(),
                "rm" => rm_process(args),
                "rmdir" => rmdir_process(args),
                "touch" => touch_process(args),
                command => command_functions(
                    command,
                    args,
                    &mut previous_command,
                    &mut commands,
                ),
            }
        }

        if is_background {
            if let Some(final_command) = previous_command {
                job_manager.add_job(final_command, input.to_string());
            }
        } else if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }
    }
}
