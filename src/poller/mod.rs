//!  The polling interface

pub mod gitlab;

/// For the poller to work. the poller will poll for the latest commits inside a repo
/// when it sees a changed commit. it quickly triggers a pipeline action to deploy to the host.
/// 
/// it represents
pub type CommitID = String;

pub trait Poller {
    fn get_latest(&self) -> CommitID;
}

pub struct RepoPoller<P: Poller> {
    pub commit: CommitID,
    poller: P,
}

impl<P: Poller> RepoPoller<P> {
    pub fn new(p: P) -> Self {
        RepoPoller {
            commit: p.get_latest(),
            poller: p,
        }
    }

    pub fn has_changed(&mut self) -> bool {
        let commit_id: CommitID = self.poller.get_latest();
        if self.commit != commit_id {
            self.commit = commit_id;
            return true;
        }
        false
    }
}


#[cfg(test)]
mod poller_tests {

    use super::*;

    struct DummyPoller;

    impl Poller for DummyPoller {
        fn get_latest(&self) -> CommitID {
            return "old commit".to_owned();
        }
    }

    #[test]
    fn test_get_latest() {
        let poller = DummyPoller;
        let mut repo : RepoPoller<DummyPoller> = RepoPoller::new(poller);
        assert!(!repo.has_changed())
    }
}