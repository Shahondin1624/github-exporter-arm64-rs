use crate::data::{RepositoriesWithCommits};

pub fn extract_number_of_repositories(data: &RepositoriesWithCommits) -> i128 {
    data.data.len() as i128
}

pub fn extract_number_of_commits_per_repository(data: &RepositoriesWithCommits) -> Vec<(String, i128)> {
    data.data.iter().map(|value| {
        (value.repository.name.clone(), value.commits.len() as i128)
    }).collect()
}

pub fn extract_total_number_of_commits(data: &RepositoriesWithCommits) -> i128 {
    data.data.iter()
        .map(|repository| &repository.commits)
        .map(|commits| commits.len() as i128).sum()
}

pub fn extract_number_of_additions_per_commit(data: &RepositoriesWithCommits) -> Vec<(String, String, i128)> {
    data.data.iter()
        .flat_map(|repository|{
            let repo_name = repository.repository.name.clone();
            repository.commits.iter()
                .map(move |commit| {
                    let sha = commit.commit.sha.clone();
                    let additions = commit.changes.stats.additions;
                    (repo_name.clone(), sha.clone(), additions as i128)
                })
        }).collect()
}

pub fn extract_number_of_deletions_per_commit(data: &RepositoriesWithCommits) -> Vec<(String, String, i128)> {
    data.data.iter()
        .flat_map(|repository|{
            let repo_name = repository.repository.name.clone();
            repository.commits.iter()
                .map(move |commit| {
                    let sha = commit.commit.sha.clone();
                    let deletions = commit.changes.stats.deletions;
                    (repo_name.clone(), sha.clone(), deletions as i128)
                })
        }).collect()
}

pub fn extract_total_number_of_additions(data: &RepositoriesWithCommits) -> i128 {
    data.data.iter()
        .map(|repository| &repository.commits)
        .map(|commits| commits.iter()
            .map(|commit| commit.changes.stats.additions as i128).sum::<i128>()).sum()
}

pub fn extract_total_number_of_deletions(data: &RepositoriesWithCommits) -> i128 {
    data.data.iter()
        .map(|repository| &repository.commits)
        .map(|commits| commits.iter()
            .map(|commit| commit.changes.stats.deletions as i128).sum::<i128>()).sum()
}