use anyhow::bail;

pub struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(&str) -> anyhow::Result<()>,
}

fn hello_command(_program: &str) -> anyhow::Result<()> {
    println!("Hello, World!");
    Ok(())
}

fn help_text(program: &str) -> String {
    let mut help = String::new();
    help.push_str(&format!("Usage: {program} <command>\nCommands:\n"));
    for Command { name, description, .. } in COMMANDS.iter() {
        help.push_str(&format!("    {name} - {description}\n"));
    }
    help
}

fn help_command(program: &str) -> anyhow::Result<()> {
    eprint!("{}", help_text(program));
    Ok(())
}

pub struct Commands<'a>(&'a [Command]);

impl<'a> Commands<'a> {
    const fn new(commands: &'a [Command]) -> Self {
        Commands(commands)
    }

    fn iter(&self) -> core::slice::Iter<'_, Command> {
        self.0.iter()
    }

    fn get(&self, command_name: &str) -> Option<&Command> {
        self.iter().find(|cmd| cmd.name == command_name)
    }
}

const COMMANDS: Commands = Commands::new(&[
    Command { name: "hello", description: "Prints \"Hello, World!\"", run: hello_command },
    Command { name: "help", description: "Print this help message", run: help_command },
]);

fn _main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let program = args.next().expect("program");
    let Some(command_name) = args.next() else {
        bail!("{}ERROR: no command was provided", help_text(&program));
    };
    let Some(command) = COMMANDS.get(&command_name) else {
        bail!("{}ERROR: command \"{command_name}\" is unknown", help_text(&program));
    };
    (command.run)(&program)
}

fn main() {
    // remove default "Error: " label
    if let Err(e) = _main() {
        eprintln!("{:?}", e);
    }
}
