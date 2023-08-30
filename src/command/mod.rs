use std::path::PathBuf;
use structopt::StructOpt;

pub const VERSION: &'static str = "0.0.1";

pub const USAGE: &'static str = "
USAGE:
ghost --branch=main --config='./' <project_name>

Flags:
-h --help       Prints the usage manual.
-v --version    Prints the ghost version.
-b --branch     Specified branch on gitlab that the ghost will run on.
-c --config     Specified config directory
";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ghost", 
    about = "A deployer for the poor"
)]
pub struct GhostCommand {
    #[structopt(short = "v", long = "version")]
    pub(crate) version: bool,
    #[structopt(short = "h", long = "help")]
    pub(crate) help: bool,
    #[structopt(short = "b", long = "branch", default_value = "main")]
    pub(crate) branch: String,
    #[structopt(short = "c", long = "configdir", default_value = "./", parse(from_os_str))]
    pub(crate) config: PathBuf,
    #[structopt(name = "project", default_value = "")]
    pub(crate) project_name: String,
    #[structopt(short = "t", long= "token", default_value = "")]
    pub(crate) gitlab_token: String
}

pub fn parse() -> GhostCommand {
    let command = GhostCommand::from_args();
    return command;
}
