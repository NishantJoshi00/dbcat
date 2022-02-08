pub mod functions {
    use std::collections::HashMap;
    use cli_table::{Cell, Table, Style};
    use colorize::AnsiColor;
    use std::error::Error;

    pub fn ttyprint(data: HashMap<String, Vec<Vec<String>>>, format: String) -> Result<(), Box<dyn Error>> {
        match format.to_lowercase().as_str() {
            "table" => {
                for (table, values) in data {
                    println!();
                    println!("Table name: {}", table.green().bold());
                    
                    let printtable: Vec<Vec<_>> = values.iter().skip(1)
                                    .map(|row| row.iter().map(|value| value.cell()).collect())
                                    .collect();
                    let otable = printtable.table()
                                    .title(values[0].iter().map(|v| v.cell().bold(true)))
                                    .bold(true);
                    
                    cli_table::print_stdout(otable)?;
                }
            },
            "raw" => {
                for (table, values) in data {
                    println!();
                    println!("{}", table);
                    for rows in values {
                        println!("{}", rows.join("\t"));
                    }
                }
            },
            _ => {

            }

        }

        Ok(())
    }
}