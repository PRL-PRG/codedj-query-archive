use std::path::Path;

use djanco::data::*;
use djanco::log::*;
use djanco::csv::*;

use djanco_ext::*;

/**
 * Gathers basic information about projects in the dataset and save it to `projects.csv`.
 */
#[djanco]
pub fn project_summary(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects().into_csv_in_dir(output, "projects.csv")
}