use serde::{Deserialize, Serialize};
use std::fs::canonicalize;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum Runner {
    ShellScript(PathBuf),
    Docker(String),
    NoOperation,
}

#[derive(Debug)]
pub enum RunnerError {
    InvalidConfig,
    ErrorRunning,
    ConfigNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunnerValue {
    command: String,
    arg: String,
}

impl From<&RunnerValue> for Runner {
    fn from(value: &RunnerValue) -> Runner {
        match value.command.as_str() {
            "shell" => {
                let full_path: PathBuf =
                    canonicalize::<PathBuf>(value.arg.to_owned().into()).unwrap();
                Runner::ShellScript(full_path)
            }
            "docker" => Runner::Docker(value.arg.to_owned()),
            _ => Runner::NoOperation,
        }
    }
}

impl RunnerValue {
    pub fn from_config(path: PathBuf) -> Result<RunnerValue, RunnerError> {
        let value = canonicalize::<PathBuf>(path.to_owned().into()).unwrap();
        log::info!("{:?}", value);
        let file = File::open(value)
            .map_err(|_| RunnerError::ConfigNotFound)?;
        serde_yaml::from_reader(file).map_err(|_| RunnerError::InvalidConfig)
    }
}

impl Runner {
    pub fn run(&self) -> () {
        match &self {
            &Runner::ShellScript(_e) => {
                // Shell Script
                let command = Command::new("bash")
                    .arg("-c")
                    .arg(_e.to_owned())
                    .output()
                    .expect("Unable to execute the command");
                let output = command.stdout;
                log::info!("{:?}", unsafe {
                    String::from_utf8_unchecked(output.to_owned())
                });
            }
            &Runner::NoOperation => {
                log::info!("No operation");
            }
            &Runner::Docker(_e) => {
                let command = Command::new("docker")
                    .arg("run")
                    .arg(_e)
                    .output()
                    .expect("Unable to run the image");
                let output = command.stdout;
                log::info!("{:?}", unsafe {
                    String::from_utf8_unchecked(output.to_owned())
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::matches;

    #[test]
    fn test_runner_value_converstion_no_op() {
        let runner_value = &RunnerValue {
            command: "".to_owned(),
            arg: "".to_owned(),
        };
        assert!(matches!(runner_value.into(), Runner::NoOperation))
    }

    #[test]
    fn test_runner_value_converstion_docker() {
        let image = String::from("repo/image");
        let runner_value = &RunnerValue {
            command: "docker".to_owned(),
            arg: image,
        };

        assert!(matches!(runner_value.into(), Runner::Docker(image)))
    }

    #[test]
    fn test_runner_value_converstion_shell() {
        let command = String::from("bash -c ./deploy.sh");
        let runner_value = &RunnerValue {
            command: "shell".to_owned(),
            arg: command,
        };

        assert!(matches!(runner_value.into(), Runner::ShellScript(command)))
    }
}
