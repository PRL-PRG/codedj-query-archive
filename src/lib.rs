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

//#[djanco(June, 2021, subsets(Python))]
pub fn all_python_projects(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        // Select all projects which are have Python declared as their major language in GitHub.
        .filter_by(Equal(project::Language, Language::Python)) 
        // Print out basic project summary for each selected project as a line in a CSV file.
        .into_csv_in_dir(output, "all_python_projects.csv")
}

//#[djanco(June, 2021, subsets(Python))]
pub fn all_projects_containing_python(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        // Select all projects which have Python as one of their constituent languages in GitHub: there's at least one
        // change to a file whose extension suggests it's a Python file: py, pyi, pyc, pyd, pyo, pyw, pyz
        .filter_by(Within(project::Languages, Language::Python)) 
        .into_csv_in_dir(output, "all_projects_containing_python_files.csv")
}

#[djanco(June, 2021, subsets(Python))]
pub fn python_snapshots_before_dec2008(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {

    let changes = database.commits() 
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
        .filter_by(Exists(change::SnapshotId));     
    
    // Prepare a subdirectory in the output folder to output the snapshot contents into.
    let mut snapshot_dir = PathBuf::from(output);
    snapshot_dir.push("python_snapshots_before_dec_2008");
    create_dir_all(snapshot_dir.clone())?;

    // A vector for gathering mappings between snapshot IDs and original file paths, for reference.
    let mut mapping_between_snapshots_and_paths: Vec<(SnapshotId, String)> = Vec::new();
    
    // Output the snapshot files into a subdirectory. The name of each snapshot will be the same as their snapshot IDs with no extensions.
    // Also: collect mapping between snapshots and original file locations.
    for change in changes {
        if let Some(snapshot) = change.snapshot() {
            // Figure out file path for snapshot contents.
            let mut file_path = snapshot_dir.clone();
            file_path.push(format!("{}", snapshot.id()));

            // write the contents of the snapshot into the file (preserves original encoding).
            let mut file = OpenOptions::new().write(true).open(file_path)?;
            file.write_all(snapshot.raw_contents())?;

            // Record a mapping between the snapshot contents file and the original path.
            let location = change.path().map_or_else(String::new, |path| path.location());
            mapping_between_snapshots_and_paths.push((snapshot.id(), location));
        }
    }

    // Output the relation between file paths and Snapshot IDs into a CSV file for reference.
    mapping_between_snapshots_and_paths.into_iter()
        // Save the relation to a CSV file.
        .into_csv_with_headers_in_dir(
            vec!["snapshot_id", "location"], 
            output, 
            "python_changes_before_dec_2008.csv")
}
