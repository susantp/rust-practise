use std::fs;
use std::io::Write;

pub trait Command {
    fn handle(&self) -> i32;
}

// Add command
pub struct AddCommand {
    args: Vec<String>,
}

pub struct ListCommand {}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        return AddCommand {
            args
        };
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description_option = &self.args.get(2);
        return if let Some(description) = description_option {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("storage.txt")
                .expect("File not found");
            writeln!(file, "{}", description)
                .expect("File not writable");
            println!("Todo added");


            0
        } else {
            println!("Description is required.");

            1
        };
    }
}

//List command
impl ListCommand {
    pub fn new() -> Self {
        return ListCommand {};
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let contents = fs::read_to_string("storage.txt")
            .expect("File not found");
        println!("{contents}");
        0
    }
}

//tests

#[cfg(test)]
mod tests {
    use crate::commands::{AddCommand, Command, ListCommand};

    #[test]
    fn add_command() {
        let args = vec![
            "todo".to_string(),
            "add".to_string(),
            "My todo 1".to_string(),
        ];
        let command: AddCommand = AddCommand::new(args);
        let exit_code: i32 = command.handle();
        assert_eq!(0, exit_code);
    }

    #[test]
    fn list_command() {
        let command: ListCommand = ListCommand::new();
        let exit_code: i32 = command.handle();
        assert_eq!(0, exit_code);
    }
}