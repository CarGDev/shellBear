use std::process::{Child, Command, Stdio};

pub fn command_functions<'a>(
    command: &str,
    args: impl Iterator<Item = &'a str>,
    previous_command: &mut Option<Child>,
    commands: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>,
) {
    let stdin = previous_command
        .take()
        .map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
        });

    let stdout = if commands.peek().is_some() {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    let output = Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();

    match output {
        Ok(output) => {
            *previous_command = Some(output);
        }
        Err(e) => {
            *previous_command = None;
            eprintln!("{}", e);
        }
    };
}
