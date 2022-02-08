pub mod functions {
    use std::error::Error;

    use colorize::AnsiColor;
    use sqlite;
    pub struct Database {
        connection: sqlite::Connection,
    }

    impl Database {
        pub fn new(path: &str) -> Result<Database, sqlite::Error> {
            let connection = sqlite::open(path)?;
            Ok(Database { connection })
        }
        pub fn get_table_names(&self) -> Result<Vec<String>, Box<dyn Error>> {
            let mut tables = Vec::new();
            self.connection.iterate(
                "SELECT name FROM sqlite_schema WHERE type='table'",
                |pair| {
                    for &(_column, value) in pair.iter() {
                        tables.push(
                            value
                                .unwrap_or("null".to_string().b_black().as_str())
                                .to_string(),
                        );
                    }
                    true
                },
            )?;

            Ok(tables)
        }

        fn get_columns(&self, table: String) -> Result<Vec<String>, Box<dyn Error>> {
            let mut stmt = self
                .connection
                .prepare(format!("pragma table_info({})", table))?
                .into_cursor();
            let mut columns: Vec<String> = Vec::new();

            while let Some(row) = stmt.next().unwrap() {
                columns.push(
                    row[1]
                        .as_string()
                        .unwrap_or("null".to_string().b_black().as_str())
                        .to_string(),
                );
            }

            Ok(columns)
        }

        pub fn get_data(
            &self,
            table: String,
            filter: Option<String>,
        ) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
            let mut data: Vec<Vec<String>> = Vec::new();

            let stmt = match filter {
                Some(fil) => format!("SELECT * FROM {} WHERE {}", table, fil),
                None => format!("SELECT * FROM {}", table),
            };

            data.push(self.get_columns(table)?);

            self.connection.iterate(stmt, |pair| {
                let mut row = Vec::new();
                for &(_column, value) in pair.iter() {
                    row.push(
                        value
                            .unwrap_or("null".to_string().b_black().as_str())
                            .to_string(),
                    );
                }
                data.push(row);
                true
            })?;

            Ok(data)
        }
    }
}
