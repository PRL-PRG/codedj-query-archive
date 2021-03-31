use std::path::Path;

use djanco::*;
use djanco::data::*;
use djanco::log::*;
use djanco::csv::*;

use djanco_ext::*;

#[djanco(April, 2020, subsets(SmallProjects))]
pub fn my_query(database: &Database, _log: &Log, output: &Path) -> Result<(), std::io::Error>  {
    database.projects()
        .group_by(project::Language)
        .sort_by(project::Stars)
        .sample(Top(1000))
        .into_csv_in_dir(output, "top_1000_small_projects_by_stars.csv")
}