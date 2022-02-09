use atty;
use clap::Parser;
use std::path::Path;
use std::{collections::HashMap, error::Error};
mod db;
mod special_print;

use colorize::AnsiColor;
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

    /// View table names from the database
    #[clap(short, long, name = "list-tables")]
    list_tables: bool,
    /// Filter the table by the given clause (placed after WHERE) (optional)
    #[clap(short, long)]
    query: Option<String>,

    /// Format the output, available formats: ["raw", "table"] (optional)
    #[clap(short, long)]
    format: Option<String>,

    /// Database File
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let path = Path::new(&args.file);

    if !path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Error: Couldn't find {}", path.display()),
        )));
    }

    let db = Database::new(&args.file)?;
    let tables = db.get_table_names()?;

    if args.list_tables {
        for table in tables {
            println!("{}", table.green().bold());
        }
        return Ok(());
    }

    let mut output: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    let filter = args.query;
    if let Some(requested_tables) = args.table {
        for table in requested_tables {
            if tables.contains(&table) {
                output.insert(table.clone(), db.get_data(table, filter.clone())?);
            }
        }
    } else {
        for table in tables {
            output.insert(table.clone(), db.get_data(table, filter.clone())?);
        }
    }

    if let Some(format) = args.format {
        ttyprint(output, format)?;
    } else {
        if atty::is(atty::Stream::Stdout) {
            ttyprint(output, "table".to_string())?;
        } else {
            ttyprint(output, "raw".to_string())?;
        }
    }
    Ok(())
}
