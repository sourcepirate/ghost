use gitlab::api::projects::repository::commits::Commits;
use gitlab::api::projects::Project;
use gitlab::api::Query;
use gitlab::types::Commit;
use gitlab::types::Project as GitlabProject;
use gitlab::Gitlab;
use crate::poller::{Poller, CommitID};


#[derive(Debug)]
pub struct GitlabPoller {
    gitlab: Gitlab,
    project: String,
    branch_name: String
}

#[derive(Debug)]
pub enum GitlabPollerError {
    AuthError,
    FetchFailed,
}

impl GitlabPoller {
    pub fn new(token: String, project: String, branch_name: String) -> Result<Self, GitlabPollerError> {
        Gitlab::new("gitlab.com".to_owned(), token.to_owned())
            .map(|e| GitlabPoller {
                gitlab: e,
                project: project,
                branch_name: branch_name
            })
            .map_err(|_| GitlabPollerError::AuthError)
    }

    pub fn get_info(&self) -> Result<GitlabProject, GitlabPollerError> {
        let endpoint = Project::builder()
            .project(self.project.as_str())
            .build()
            .map_err(|_| GitlabPollerError::FetchFailed)?;
        endpoint
            .query(&self.gitlab)
            .map_err(|_| GitlabPollerError::FetchFailed)
    }

    pub fn get_commits(&self, branch_name: &str) -> Result<Vec<Commit>, GitlabPollerError> {
        let endpoint = Commits::builder()
            .project(self.project.as_str())
            .ref_name(branch_name)
            .build()
            .map_err(|_| GitlabPollerError::FetchFailed)?;
        endpoint
            .query(&self.gitlab)
            .map_err(|_| GitlabPollerError::FetchFailed)
    }

    pub fn get_latest_commit(&self) -> Result<Option<Commit>, GitlabPollerError> {
        self.get_commits(self.branch_name.as_str()).map(|c| c.into_iter().next())
    } 

}


impl Poller for GitlabPoller {

    fn get_latest(&self) -> CommitID {
        match self.get_latest_commit() {
            Ok(_e) => _e.map(|i| i.short_id.unwrap()).unwrap_or_else(|| "".to_owned()),
            Err(_) => "".to_owned()
        }
    }
}