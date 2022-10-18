use std::fmt::Display;

use git2::{ErrorCode, Repository};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct VersionInfo {
    branch: String,
    revision: String,
    time: OffsetDateTime,
}

impl Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({})", self.branch, self.revision, self.time)
    }
}

/// Retrieves the current git branch in a given git repository.
pub fn show_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return format!("An error occured: {:?}", e),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the given git repository.
pub fn show_head_rev(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}

// get date from latest revision
pub fn show_head_rev_date(repo: &Repository) -> OffsetDateTime {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();

    OffsetDateTime::from_unix_timestamp(revision.as_commit().unwrap().time().seconds()).unwrap()
}

pub fn get_version() -> VersionInfo {
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR")).expect("Error opening .git");
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);
    let time = show_head_rev_date(&repo);

    VersionInfo {
        branch,
        revision,
        time,
    }
}
