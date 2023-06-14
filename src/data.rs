use std::hash::{Hash, Hasher};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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

impl Hash for MinimalRepository {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.node_id.hash(state);
        self.name.hash(state);
        self.full_name.hash(state);
        self.owner.hash(state);
        self.private.hash(state);
        self.html_url.hash(state);
        self.description.hash(state);
        self.fork.hash(state);
        self.url.hash(state);
        self.archive_url.hash(state);
        self.assignees_url.hash(state);
        self.blobs_url.hash(state);
        self.branches_url.hash(state);
        self.collaborators_url.hash(state);
        self.comments_url.hash(state);
        self.commits_url.hash(state);
        self.compare_url.hash(state);
        self.contents_url.hash(state);
        self.contributors_url.hash(state);
        self.deployments_url.hash(state);
        self.downloads_url.hash(state);
        self.events_url.hash(state);
        self.forks_url.hash(state);
        self.git_commits_url.hash(state);
        self.git_refs_url.hash(state);
        self.git_tags_url.hash(state);
        self.git_url.hash(state);
        self.issue_comment_url.hash(state);
        self.issue_events_url.hash(state);
        self.issues_url.hash(state);
        self.keys_url.hash(state);
        self.labels_url.hash(state);
        self.languages_url.hash(state);
        self.merges_url.hash(state);
        self.milestones_url.hash(state);
        self.notifications_url.hash(state);
        self.pulls_url.hash(state);
        self.pulls_url.hash(state);
        self.releases_url.hash(state);
        self.ssh_url.hash(state);
        self.stargazers_url.hash(state);
        self.statuses_url.hash(state);
        self.subscribers_url.hash(state);
        self.subscription_url.hash(state);
        self.tags_url.hash(state);
        self.teams_url.hash(state);
        self.trees_url.hash(state);
        self.clone_url.hash(state);
        self.mirror_url.hash(state);
        self.hooks_url.hash(state);
        self.svn_url.hash(state);
        self.homepage.hash(state);
        self.language.hash(state);
        self.forks_count.hash(state);
        self.stargazers_count.hash(state);
        self.watchers_count.hash(state);
        self.size.hash(state);
        self.default_branch.hash(state);
        self.open_issues_count.hash(state);
        self.is_template.hash(state);
        self.topics.hash(state);
        self.has_issues.hash(state);
        self.has_projects.hash(state);
        self.has_wiki.hash(state);
        self.has_pages.hash(state);
        self.has_downloads.hash(state);
        self.has_discussions.hash(state);
        self.archived.hash(state);
        self.disabled.hash(state);
        self.visibility.hash(state);
        self.pushed_at.hash(state);
        self.created_at.hash(state);
        self.updated_at.hash(state);
        if let Some(permissions) = &self.permissions {
            permissions.to_string().hash(state);
        }
        if let Some(template_repository) = &self.template_repository {
            template_repository.to_string().hash(state);
        }
        self.temp_clone_token.hash(state);
        self.delete_branch_on_merge.hash(state);
        self.subscribers_count.hash(state);
        self.network_count.hash(state);
    }
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct FullCommitData {
    pub commit: Commit,
    pub changes: CommitChangeDetails,
}

impl FullCommitData {
    pub fn from(commit: Commit, changes: CommitChangeDetails) -> FullCommitData {
        FullCommitData {
            commit,
            changes,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct RepositoriesWithCommits {
    pub data: Vec<RepositoryAndCommits>,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct CommitDetails {
    pub url: String,
    pub author: Option<GitUser>,
    pub committer: Option<GitUser>,
    pub message: String,
    pub comment_count: i32,
    pub tree: Tree,
    pub verification: Verification,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub payload: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    User,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct CommitStats {
    pub additions: i32,
    pub deletions: i32,
    pub total: i32,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct CommitChangeDetails {
    pub stats: CommitStats,
    pub files: Vec<DiffEntry>,
}


