#[macro_use]
extern crate serde_json;

extern crate docopt;
extern crate gitlab;
extern crate serde;
extern crate log;

use log::{info, warn, error};


pub mod poller;
pub mod command;


fn main() {
    // match poller::gitlab::GitlabPoller::new(
    //     "glpat-FGiW-CevGxu_wFFpW91z".to_owned(), "kalkiai/indexer".to_owned()
    // ) {
    //     Ok(poller) =>  println!("{:?}", poller.get_latest_commit("master")),
    //     Err(_) => println!("Erroed")
    // }

    if let Ok(cmd) =  command::parse() {
        info!("Parsing command line: {:?}", cmd);
    } else {
        error!("Cannot parse the command line args")
    }

}
