use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimpleUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MinimalRepository {
    pub id: i32,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: SimpleUser,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: Option<String>,
    pub branches_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url: Option<String>,
    pub commits_url: Option<String>,
    pub compare_url: Option<String>,
    pub contents_url: Option<String>,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub labels_url: Option<String>,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url: Option<String>,
    pub releases_url: Option<String>,
    pub ssh_url: Option<String>,
    pub stargazers_url: String,
    pub statuses_url: Option<String>,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: Option<String>,
    pub clone_url: Option<String>,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub size: i32,
    pub default_branch: String,
    pub open_issues_count: i32,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub has_discussions: bool,
    pub archived: Option<bool>,
    pub disabled: Option<bool>,
    pub visibility: Option<String>,
    pub pushed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub permissions: Option<serde_json::Value>,
    pub template_repository: Option<serde_json::Value>,
    pub temp_clone_token: Option<String>,
    pub delete_branch_on_merge: Option<bool>,
    pub subscribers_count: Option<i32>,
    pub network_count: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FullCommitData {
    pub commit: Commit,
    pub changes: CommitChangeDetails
}

impl FullCommitData {
    pub fn from(commit: Commit, changes: CommitChangeDetails) -> FullCommitData {
        FullCommitData {
            commit,
            changes,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct RepositoryAndCommits {
    pub repository: MinimalRepository,
    pub commits: Vec<FullCommitData>,
}

impl RepositoryAndCommits {
    pub fn from(repository: MinimalRepository, commits: Vec<FullCommitData>) -> RepositoryAndCommits {
        RepositoryAndCommits {
            repository,
            commits,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Commit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitDetails,
    pub author: Option<SimpleUser>,
    pub committer: Option<SimpleUser>,
    pub parents: Vec<Parent>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CommitDetails {
    pub url: String,
    pub author: Option<GitUser>,
    pub committer: Option<GitUser>,
    pub message: String,
    pub comment_count: i32,
    pub tree: Tree,
    pub verification: Verification,
}

#[derive(Debug, serde::Deserialize)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub payload: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    User,
}

#[derive(Debug, serde::Deserialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitStats {
    pub additions: i32,
    pub deletions: i32,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct DiffEntry {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: i32,
    pub deletions: i32,
    pub changes: i32,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    pub patch: Option<String>,
    pub previous_filename: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CommitChangeDetails {
    pub stats: CommitStats,
    pub files: Vec<DiffEntry>,
}


