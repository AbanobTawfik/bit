mod init;

#[derive(Debug, Clone)]
pub struct CommandArgs {

}

#[derive(Debug, Clone, Copy)]
pub enum CommandStatus {
    Success,
    Error,
}

trait Command {
    fn run(args: &CommandArgs) -> CommandStatus;
}