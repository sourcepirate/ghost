use docopt::Docopt;
use docopt::Error;
use serde::Deserialize;

const VERSION: &'static str = "0.0.1";

const USAGE: &'static str = "
Ghost runner

Usage:
  Ghost haunt <project_name> --branch main --config config
  Ghost --version
  Ghost --help

Options:
    -h --help                   Show help screen.
    --version                   Show version.
    --branch=<branch_name>      Branch to run the deployer on [default: main]
    --config=<path>             Location of ghost.yaml [default: ./]
";

#[derive(Debug, Deserialize)]
pub struct GhostCommand {
    pub(crate) cmd_haunt: bool,
    pub(crate) flag_version: bool,
    pub(crate) flag_help: bool,
    pub(crate) flag_branch: String,
    pub(crate) flag_config: String,
    pub(crate) project_name: String,
}

pub fn parse() -> Result<GhostCommand, Error> {
    Docopt::new(USAGE).and_then(|d| d.deserialize())
}
