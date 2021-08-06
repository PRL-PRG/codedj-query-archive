use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs::create_dir_all;

//use chrono::*;

use djanco::*;
use djanco::database::*;
use djanco::log::*;
use djanco::csv::*;

use djanco::objects::*;

use djanco_ext::*;

// #[djanco(subsets(Python, SmallProjects))]
// pub fn all_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.projects()
//         // Print out basic project summary for each selected project as a line in a CSV file.
//         .into_csv_in_dir(output, "all_projects.csv")
// }

// #[djanco(subsets(Python, SmallProjects))]
// pub fn all_python_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.projects()
//         // Select all projects which are have Python declared as their major language in GitHub.
//         .filter_by(Equal(project::Language, Language::Python)) 
//         // Print out basic project summary for each selected project as a line in a CSV file.
//         .into_csv_in_dir(output, "all_python_projects.csv")
// }

// #[djanco(subsets(Python, SmallProjects))]
// pub fn all_projects_containing_python(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.projects()
//         // Select all projects which have Python as one of their constituent languages in GitHub: there's at least one
//         // change to a file whose extension suggests it's a Python file: py, pyi, pyc, pyd, pyo, pyw, pyz
//         .filter_by(Within(project::Languages, Language::Python)) 
//         .into_csv_in_dir(output, "all_projects_containing_python_files.csv")
// }

// //#[djanco(subsets(Python, SmallProjects))]
// pub fn all_commits(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits()
//         .into_csv_in_dir(output, "all_commits.csv")
// }

// //#[djanco(subsets(Python, SmallProjects))]
// pub fn all_changes(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits()
//         .map_into(commit::Changes)
//         .flat_map(|option| option)
//         // Auxiliary: flatten from a stream of vectors of changes to a stream of changes.
//         .flat_map(|vector| vector)
//         // Select changes where the changed file has the extensions associated with Python.
//         .into_csv_in_dir(output, "all_changes.csv")
// }

// //#[djanco(subsets(Python, SmallProjects))]
// pub fn all_paths(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits()
//         .map_into(commit::Paths)
//         .flat_map(|option| option)
//         // Auxiliary: flatten from a stream of vectors of changes to a stream of changes.
//         .flat_map(|vector| vector)
//         // Select changes where the changed file has the extensions associated with Python.
//         .into_csv_in_dir(output, "all_paths.csv")
// }

// //#[djanco(subsets(Python, SmallProjects))]
// pub fn all_snapshot_ids(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits()
//         .map_into(commit::SnapshotIds)
//         .flat_map(|option| option)
//         // Auxiliary: flatten from a stream of vectors of changes to a stream of changes.
//         .flat_map(|vector| vector)
//         // Select changes where the changed file has the extensions associated with Python.
//         .into_csv_in_dir(output, "all_snapshot_ids.csv")
// }

// #[djanco(subsets(Python, SmallProjects))]
// pub fn python_snapshots_debug(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits().map(|commit| {
//         let hash = commit.hash().unwrap_or_else(String::new);

//         let change_count = commit.change_count().unwrap_or(0);

//         let languages = commit.languages().unwrap_or_else(Vec::new);
//         let is_python = languages.contains(&Language::Python);

//         let timestamp = commit.author_timestamp();
//         let dec_2008 = timestamp!(December 2008);
//         let before_dec_2008 = timestamp.map_or(false, |date| {
//             date < timestamp!(December 2008)
//         });

//         //let date = timestamp.map(|t| /
//         //    t.as_utc_rfc2822_string()
//         //}).unwrap_or_else(String::new);

//         (hash, change_count, is_python, timestamp, dec_2008, before_dec_2008)//, date)
//     }).into_csv_in_dir(output, "python_commit_debug.csv")
// }

// #[djanco(subsets(Python, SmallProjects))]
// pub fn python_snapshots_debug_interesting(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     database.commits().flat_map(|commit| {
//         let hash = commit.hash().unwrap_or_else(String::new);

//         let change_count = commit.change_count().unwrap_or(0);

//         let languages = commit.languages().unwrap_or_else(Vec::new);
//         let is_python = languages.contains(&Language::Python);

//         let timestamp = commit.author_timestamp();
//         let dec_2008 = timestamp!(December 2008);
//         let before_dec_2008 = timestamp.map_or(false, |date| {
//             date < timestamp!(December 2008)
//         });

//         //let date = timestamp.map(|t| /
//         //    t.as_utc_rfc2822_string()
//         //}).unwrap_or_else(String::new);

//         if is_python && before_dec_2008 {
//             let changes = commit.changes_with_data().map_or_else(Vec::new, |changes| {
//                 changes.into_iter().map(|change| {
//                     let path = change.path_with_data().map_or_else(|| String::from("<missing>"), |path| path.location());
//                     let has_snapshot = change.snapshot_with_data().is_some();
//                     // if has_snapshot {
//                     //     println!("{}", change.snapshot_with_data().unwrap().contents());
//                     //     println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
//                     // }
//                     // (path, has_snapshot)
//                 }).collect::<Vec<(String, bool)>>()
//             });
//             Some((hash, change_count, is_python, timestamp, dec_2008, before_dec_2008, changes))
//         } else {
//             None
//         }
//     }).into_csv_in_dir(output, "python_commit_debug_just_interesting_commits.csv")
// }

// #[djanco(subsets(Python, SmallProjects))]
// pub fn python_snapshots_store(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
//     let mut snapshot_dir = PathBuf::from(output);
//     snapshot_dir.push("python_snapshots_before_dec2008-2");

//     database.commits().flat_map(|commit| {
//         let hash = commit.hash().unwrap_or_else(String::new);

//         let change_count = commit.change_count().unwrap_or(0);

//         let languages = commit.languages().unwrap_or_else(Vec::new);
//         let is_python = languages.contains(&Language::Python);

//         let timestamp = commit.author_timestamp();
//         // let dec_2008 = timestamp!(December 2008);
//         let before_dec_2008 = timestamp.map_or(false, |date| {
//             date < timestamp!(December 2008)
//         });

//         //let date = timestamp.map(|t| {
//         //    t.as_utc_rfc2822_string()
//         //}).unwrap_or_else(String::new);

//         if is_python && before_dec_2008 {

//             println!("commit {} contains Python from before Dec 2008", hash);

//             let changes: Vec<ItemWithData<Change>> = commit.changes_with_data().unwrap_or_else(Vec::new);
//             let python_changes: Vec<ItemWithData<Change>> = changes.into_iter()
//                 .filter(|change| {
//                     change.path().map_or(false, |path| {
//                         path.language().map_or(false, |language| {
//                             language == Language::Python
//                         })
//                     })
//                 })
//                 .collect();
//             let python_changes_count = python_changes.len();
//             let python_snapshots: Vec<ItemWithData<Snapshot>> = python_changes.into_iter()
//                 .flat_map(|change| {
//                     change.snapshot_with_data()
//                 })
//                 .collect();
//             let python_snapshot_count = python_snapshots.len();
                
//             println!("commit {} contains Python from before Dec 2008: {} changes, {} Python paths, {} Python snapshots", 
//                      hash, change_count, python_changes_count, python_snapshot_count);

//             python_snapshots
            
//         } else {
//             vec![]
//         }
//     }).into_files_in_dir(&snapshot_dir)
// }   

#[djanco(subsets(Python))]
pub fn python_path_map_python(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut dir = PathBuf::from(output);
    dir.push("python");
    python_path_map(database, log, &dir)
}

#[djanco(subsets(SmallProjects))]
pub fn python_path_map_small_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut dir = PathBuf::from(output);
    dir.push("small_projects");
    python_path_map(database, log, &dir)
}

pub fn python_path_map(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.commits() 
        // Select commits that occured before 1st Dec 2008. Python 3 was released December 3, 2008, Djanco standard resolution is 1 Month.
        .filter_by(LessThan(commit::AuthoredTimestamp, timestamp!(December 2008)))    
        // Select commits that changed at least one Python file. Python files are recognized by extensions: py, pyi, pyc, pyd, pyo, pyw, pyz
        .filter_by(Within(commit::Languages, Language::Python))        
        // Get changes from each of the remaining commits.
        .map_into(commit::Changes)        
        // Auxiliary: get rid of some Option wrappers, basically like removing nulls
        .flat_map(|option| option)        
        // Auxiliary: flatten from a stream of vectors of changes to a stream of changes.
        .flat_map(|vector| vector)        
        // Select changes where the changed file has the extensions associated with Python.
        .filter_by(Equal(From(change::Path, path::Language), Language::Python))
        // Discard all the changes for which we don't have a snapshot (for whatever reason)
        .filter_by(Exists(change::SnapshotId))
        // -------------- same query ----------------
        // Get a location and a corresponding snapshot id
        .map_into(Select!(From(change::Path, path::Location), change::SnapshotId))
        // Output to a CSV file.
        .into_csv_with_headers_in_dir(vec!["path", "snapshot_id"], output, "paths.csv")
}

#[djanco(subsets(Python))]
pub fn python_snapshots_python(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut dir = PathBuf::from(output);
    dir.push("python");
    python_snapshots(database, log, &dir)
}

#[djanco(subsets(SmallProjects))]
pub fn python_snapshots_small_projects(database: &Database, log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    let mut dir = PathBuf::from(output);
    dir.push("small_projects");
    python_snapshots(database, log, &dir)
}

pub fn python_snapshots(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error> {

    // Prepare a subdirectory in the output folder to output the snapshot contents into.
    let mut snapshot_dir = PathBuf::from(output);
    snapshot_dir.push("snapshots");
    create_dir_all(snapshot_dir.clone())?;

    //let x: Vec<ItemWithData<Snapshot>> = 
    database.commits() 
        // Select commits that occured before 1st Dec 2008. Python 3 was released December 3, 2008, Djanco standard resolution is 1 Month.
        .filter_by(LessThan(commit::AuthoredTimestamp, timestamp!(December 2008)))    
        // Select commits that changed at least one Python file. Python files are recognized by extensions: py, pyi, pyc, pyd, pyo, pyw, pyz
        .filter_by(Within(commit::Languages, Language::Python))        
        // Get changes from each of the remaining commits.
        .map_into(commit::Changes)        
        // Auxiliary: get rid of some Option wrappers, basically like removing nulls
        .flat_map(|option| option)        
        // Auxiliary: flatten from a stream of vectors of changes to a stream of changes.
        .flat_map(|vector| vector)        
        // Select changes where the changed file has the extensions associated with Python.
        .filter_by(Equal(From(change::Path, path::Language), Language::Python))
        // Discard all the changes for which we don't have a snapshot (for whatever reason)
        .filter_by(Exists(change::SnapshotId))        
        // -------------- same query ----------------
        // Get the file contents out of the change item
        .map_into(change::Snapshot)
        // Remove empties
        .flat_map(|option| option) 
        // Write all the file contents to files, using ids as filenames
        .into_files_in_dir(&snapshot_dir)
}
