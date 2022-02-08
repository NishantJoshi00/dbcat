use std::collections::HashMap;
use clap::Parser;
use atty;

mod db;
mod special_print;

use db::functions::Database;
use special_print::functions::ttyprint;

/// Read sql database from .db file. Select specific tables to print, filter tables based on query clause
/// 
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {

    /// Select the table from the database (optional)
    #[clap(short, long, multiple_occurrences(true))]
    table: Option<Vec<String>>,

    /// Filter the table by the given clause (placed after WHERE) (optional)
    #[clap(short, long)]
    query: Option<String>,

    /// Format the output, available formats: ["raw", "table"] (optional)
    #[clap(short, long)]
    format: Option<String>,

    /// Database File 
    file: String,
}







fn main() {
    let args = Args::parse();
    let db = Database::new(&args.file).unwrap();

    let mut output: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let tables = db.get_table_names();
    let filter = args.query;
    if let Some(requested_tables) = args.table {
        for table in requested_tables {
            if tables.contains(&table) {
                output.insert(table.clone(), db.get_data(table, filter.clone()));
            }
        }
    } else {
        for table in tables {
            output.insert(table.clone(), db.get_data(table, filter.clone()));
        }
    }

    if let Some(format) = args.format {
        ttyprint(output, format);
    } else {
        if atty::is(atty::Stream::Stdout) {
            ttyprint(output, "table".to_string());
        } else {
            ttyprint(output, "raw".to_string());
        }
    }
}
