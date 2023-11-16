use std::fs;
use std::io;

trait Command {
    // trait will be implemented by all the commands
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<(), String>;
}

struct PingCommand;
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<(), String> {
        println!("pong!");
        Ok(())
    }
}

struct CountCommand;
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), String> {
        let count = args.len();
        println!("counted {} args", count);
        Ok(())
    }
}

struct TimesCommand {
    count: usize,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _args: &[&str]) -> Result<(), String> {
        self.count += 1;
        println!("This command was called: {} time/times!", self.count);
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::new(),
        }
    }
    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    fn run(&mut self, name_of_file: &str) -> Result<(), io::Error> {
        let input_text = fs::read_to_string(name_of_file)?;

        for line in input_text.lines() {
            let lines_trim = line.trim();

            if lines_trim.is_empty() {
                continue;
            }

            let mut parts = lines_trim.split_whitespace();
            if let Some(command_name) = parts.next() {
                if command_name.to_lowercase() == "stop" {
                    break;
                }

                let args: Vec<&str> = parts.collect();
                match self
                    .commands
                    .iter_mut()
                    .find(|cmd| cmd.get_name() == command_name)
                {
                    Some(command) => {
                        if let Err(err) = command.exec(&args) {
                            println!("Errror: {}", err);
                        }
                    }
                    None => {
                        println!("Unknown command: {}", command_name);
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run("file.txt")?; //argument for run- function
    Ok(())
}
