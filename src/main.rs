extern crate gitlab;
extern crate serde;
extern crate structopt;

extern crate env_logger;
extern crate log;

use std::process::exit;

pub mod command;
pub mod poller;
pub mod executor;

use std::thread::sleep;
use std::time::Duration;
use poller::{ RepoPoller, gitlab::GitlabPoller};
use executor::{RunnerValue, Runner};

fn main() {
    env_logger::init();

    let command = command::parse();
    if command.version {
        //
        println!("v{}", command::VERSION);
        exit(0)
    } else if command.help {
        //
        println!("{}", command::USAGE);
        exit(2)
    } else {
        let poller = poller::gitlab::GitlabPoller::new(
            command.gitlab_token.to_owned(),
            command.project_name.to_owned(),
            command.branch.to_owned(),
        );
        match poller {
            Ok(pol) => {
                let mut repo : RepoPoller<GitlabPoller> = RepoPoller::new(pol);
                let runner_wrap: Result<executor::RunnerValue, executor::RunnerError> = RunnerValue::from_config(command.config.clone());
                if runner_wrap.is_err() {
                    log::error!("Invalid config file at - {:?}", command.config)
                }
                loop {
                    if repo.has_changed() {
                        let runner_value: &RunnerValue = runner_wrap.as_ref().unwrap();
                        let runner: Runner = runner_value.into();
                        log::info!("New commit detected : {}", repo.commit);
                        runner.run();
                    }
                    log::info!("Sleeping for 20 seconds");
                    sleep(Duration::new(20, 0))
                }
            },
            Err(_) => {
                log::error!("Errored Polling!");
            }
        };
    }
}
