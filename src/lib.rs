use std::path::Path;

use djanco::*;
use djanco::attrib::sort::Direction;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco::objects::CommitId;
use djanco::objects::Head;
use djanco::objects::ItemWithData;
use djanco::objects::Language;
use djanco::objects::Project;
use djanco::objects::ProjectId;

use djanco_ext::*;

// use itertools;

const SELECTED_PROJECTS: usize = 20;
const SEEDS: [u128; 10] = [1,2,3,5,7,11,13,17,19,23];

// Base commit is going to be a commit this many percent commits in the past.
//
// Eg. if there are 12 commits and BASE_COMMIT_OFFSET_RATIO is 25, then
// the base commit will be 12 * 25 / 100 = 3 commits pushed back from the head.
//
// A B C D E F G H I J K L
//                 ^     ^
//                 |     |
//                 BASE  HEAD
//
// All math is done on integers.
const BASE_COMMIT_OFFSET_RATIO: usize = 10;

#[djanco(May, 2021, subsets(JavaScript))]
pub fn all_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .into_csv_in_dir(output, "javascript_projects.csv")
}

#[djanco(May, 2021, subsets(JavaScript))]
pub fn project_locs(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
    .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
    .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
    .map_into(Select!(project::URL, project::Locs))
    .into_csv_in_dir(output, "project_locs.csv")
}

#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_0(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 0) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_1(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 1) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_2(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 2) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_3(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 3) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_4(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 4) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_5(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 5) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_6(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 6) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_7(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 7) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_8(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 8) }
#[djanco(May, 2021, subsets(JavaScript))] pub fn random_projects_9(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { random_projects(database, log, output, 9) }

pub fn random_projects(database: &Database, _log: &Log, output: &Path, seed_index: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .filter(is_project_spec)
        .sample(Random(SELECTED_PROJECTS, Seed(SEEDS[seed_index])))
        .flat_map(project_spec)
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, format!("random_projects_{}_{}.csv", seed_index, BASE_COMMIT_OFFSET_RATIO))
}

#[djanco(May, 2021, subsets(JavaScript))]
pub fn top_starred(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Substore, Store::Large(store::Language::JavaScript)))
        .filter_by(AnyIn(project::Languages, vec![Language::JavaScript]))
        .sort_by(project::Stars)
        .filter(is_project_spec)
        .flat_map(project_spec)      
        .into_csv_with_headers_in_dir(vec!["url", "to", "from"], output, "top_starred_projects.csv")
}

// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_0(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 0) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_1(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 1) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_2(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 2) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_3(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 3) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_4(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 4) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_5(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 5) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_6(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 6) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_7(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 7) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_8(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 8) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_9(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 9) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_10(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 10) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_11(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 11) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_12(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 12) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_13(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 13) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_14(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 14) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_15(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 15) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_16(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 16) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_17(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 17) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_18(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 18) }
// #[djanco(May, 2021, subsets(JavaScript))] pub fn debug_19(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  { debug(database, log, output, 19) }

pub fn debug(database: &Database, _log: &Log, output: &Path, project_id: usize) -> Result<(), std::io::Error>  {
    database.projects()
        .filter_by(Equal(project::Id, ProjectId::from(project_id)))       
        .map_into(project::Commits)
        .flat_map(|e| e )
        .flat_map(|e| e)
        .map_into(Select!(commit::Id, commit::Hash, commit::Message))
        .into_csv_in_dir(output, format!("{}_commits.csv", project_id))
}

//#[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_commits(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    database.commits()
        .sort_with_direction(Direction::Ascending, commit::Id)
        .map_into(Select!(commit::Id, commit::Hash))
        .into_csv_in_dir(output, "commits.csv") 
}
//#[djanco(May, 2021, subsets(JavaScript))]
pub fn debug_commits_from_source(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut hashes: Vec<(djanco::objects::CommitId, String)> = database.source().commit_hashes().collect();
    hashes.sort_by_key(|(id, _hash)| *id);
    hashes.into_iter().into_csv_in_dir(output, "commits_from_source.csv")
}

// #[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_heads(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    let mut heads = database.projects()
        .map_into(Select!(project::Id, project::Heads))
        .map(|(project_id, heads)| (project_id, heads.unwrap_or_else(Vec::new)))
        .map(|(project_id, heads)| (project_id, heads.into_iter().map(|head| {
                (head.name(), (head.commit_id(), head.commit_with_data().unwrap().hash().unwrap()))
            }).collect::<Vec<(String, (CommitId, String))>>())
        )
        .map(|(project_id, mut heads)| {heads.sort(); (project_id, heads)})
        .collect::<Vec<(ProjectId, Vec<(String, (CommitId, String))>)>>();
    heads.sort();
    heads.into_iter().into_csv_in_dir(output, "heads.csv")
}

// #[djanco(May, 2021, subsets(JavaScript))] 
pub fn debug_heads_from_source(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  { 
    let mut heads = database.source()
        .project_heads()
        .map(|(project_id, heads)| 
            (project_id, heads.into_iter().collect::<Vec<(String, (djanco::objects::CommitId, String))>>())
        )
        .map(|(project_id, mut heads)| {heads.sort(); (project_id, heads)})
        .collect::<Vec<(djanco::objects::ProjectId, Vec<(String, (djanco::objects::CommitId, String))>)>>();
    heads.sort();
    heads.into_iter().into_csv_in_dir(output, "heads_from_source.csv")
}

// Helper functions:
type ProjectURL = String;
type CommitHash = String;

fn is_project_spec<'a>(project: &ItemWithData<'a, Project>) -> bool {
    _project_spec(project).is_some()
}
fn project_spec<'a>(project: ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    _project_spec(&project)
}
fn _project_spec<'a>(project: &ItemWithData<'a, Project>) -> Option<(ProjectURL, CommitHash, CommitHash)> {
    let url = project.url();
    
    let default_branch = project.default_branch();
    if default_branch.is_none() {
        eprintln!("WARNING: Default branch not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let default_branch = default_branch.unwrap();
    let default_branch_path = format!("refs/heads/{}", default_branch);

    let heads = project.heads_with_data();
    if heads.is_none() {
        eprintln!("WARNING: Heads not found for project {} ({:?}), skipping.", project.id(), url);
        return None;
    }
    let heads = heads.unwrap();

    //eprintln!("INFO: heads in project {} ({:?}): {}", project.id(), url, heads.iter().map(|head| format!("{}:{}:{:?}", head.name(), head.commit_id(), head.commit_with_data().unwrap().hash().unwrap()) ).collect::<Vec<String>>().join("\n"));

    let default_branch_head = 
        heads.into_iter()
            .filter(|head| head.name() == default_branch_path)            
            .collect::<Vec<ItemWithData<Head>>>();

    if default_branch_head.len() == 0 {
        eprintln!("WARNING: No branch {} found in project {} ({:?}), skipping.", default_branch, project.id(), url);
        return None;
    }
    if default_branch_head.len() > 1 {
        eprintln!("WARNING: More than one ({}) branch {} found in project {} ({:?}), continuing.", 
                  default_branch_head.len(), default_branch, project.id(), url);
    }
    let default_branch_head = default_branch_head[0].clone();

    let head_commit = default_branch_head.commit_with_data();
    if head_commit.is_none() {
        eprintln!("WARNING: Head commit inaccessible from branch {} in project {} ({:?}), skipping.", 
                  default_branch, project.id(), url);
        return None;
    }
    let head_commit = head_commit.unwrap();
    
    let head_commit_hash = head_commit.hash();
    if head_commit_hash.is_none() {
        eprintln!("WARNING: Head commit hash unavaiable for head commit {} from branch {} in project {} ({:?}), skipping.", 
                  head_commit.id(), default_branch, project.id(), url);
        return None;
    }
    let head_commit_hash = head_commit_hash.unwrap();

    let mut commits = default_branch_head.commits_with_data();

    // Newest first.
    commits.sort_by_key(|commit| commit.committer_timestamp());
    commits.reverse();

    let total_commits = commits.len();
    let base_commit_offset = (BASE_COMMIT_OFFSET_RATIO * total_commits) / 100;

    eprintln!("INFO: Base commit offset is {} (of {}) for project {} ({:?})", 
              base_commit_offset, total_commits, project.id(), url);

    let base_commit = commits.iter().take(base_commit_offset).last();
    if base_commit.is_none() {
        eprintln!("WARNING: Base commit unavaiable for for branch {} in project {} ({:?}), skipping.", 
                  default_branch, project.id(), url);
        return None;
    }
    let base_commit = base_commit.unwrap();


    let base_commit_hash = base_commit.hash();
    if base_commit_hash.is_none() {
        eprintln!("WARNING: Base commit hash unavaiable for base commit {} from branch {} in project {} ({:?}), skipping.", 
                  base_commit.id(), default_branch, project.id(), url);
        return None;
    }
    let base_commit_hash = base_commit_hash.unwrap();

    return Some((url, head_commit_hash, base_commit_hash))
}