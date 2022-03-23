mod init;

use clap::{AppSettings, Parser, Subcommand};

#[derive(Debug, Clone)]
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CommandArgs {
    #[clap(subcommand)]
    sub_command: SubCommandArgs
}

#[derive(Debug, Clone)]
#[derive(Subcommand)]
pub enum SubCommandArgs {
    #[clap(arg_required_else_help = true)]
    Init {
        directory: String
    },
    #[clap(arg_required_else_help = true)]
    Commit {
        message: Option<String>,
        files: Vec<String>,
    },
    Branch,
    Checkout,
}

#[derive(Debug, Clone, Copy)]
pub enum CommandStatus {
    Success,
    Error,
}

trait Command {
    fn run(args: &CommandArgs) -> CommandStatus;
}