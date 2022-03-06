use super::{Command, CommandArgs, CommandStatus};

struct Init;

impl Command for Init {
    fn run(args: &CommandArgs) -> CommandStatus {
        
        
        CommandStatus::Success
    }
}