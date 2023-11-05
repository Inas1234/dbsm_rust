use std::fs::{self, File};
use std::io::BufReader;
use serde_json::{Value, Map};
use crate::parser::{NodeStmt, NodeExprIdentifier, NodeExpr, NodeProg};

pub(crate) struct Generator {
    global_db_file: Option<String>,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            global_db_file: None,
        }
    }


    pub fn generate(&mut self, prog: NodeProg) -> Result<(), Box<dyn std::error::Error>> {
        for stmt in prog.nodes {
            match stmt {
                NodeStmt::CreateDatabase(stmt) => {
                    let db_name = self.expr_to_string(&stmt.database_name)?;
                    let db_file = format!("{}.json", db_name);
                    
                    let file = File::create(&db_file)?;
                    serde_json::to_writer(&file, &serde_json::json!({}))?;
                    
                    println!("Database created: {}", db_name);
                },
                NodeStmt::UseDatabaase(stmt) => { 
                    let db_name = self.expr_to_string(&stmt.db_name)?;
                    self.global_db_file = Some(format!("{}.json", db_name));
                    println!("Using database: {}", db_name);
                    println!("Debug: global_db_file set to {:?}", self.global_db_file);
                },
                NodeStmt::CreateTable(stmt) => {
                    if let Some(_) = self.global_db_file {
                        let mut db = self.read_database()?;
                        let table_name = self.expr_to_string(&stmt.table_name)?;
                        let mut columns = Vec::new();

                        for column in stmt.columns {
                            let column_name = self.expr_to_string(&column)?;
                            columns.push(column_name);
                        }

                        let table: Value = serde_json::json!({
                            "columns": columns,
                            "rows": []
                        });
                        db.insert(table_name, table);

                        self.write_database(db)?;
                    } else {
                        return Err("No database selected. Use 'USE DATABASE' statement before creating tables.".into());
                    }
                },
                NodeStmt::ListDatabase(_stmt) => {
                    self.list_databases()?;
                },
                NodeStmt::ListTable(_stmt) => {
                    if let Some(_) = self.global_db_file {
                        self.list_tables()?;
                    } else {
                        return Err("No database selected. Use 'USE DATABASE' statement before listing tables.".into());
                    }
                }
            }
        }

        Ok(())
    }

    fn list_databases(&self) -> Result<(), Box<dyn std::error::Error>> {
        let databases: Vec<_> = fs::read_dir("./")?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.path().extension()? == "json" {
                    entry.path().file_stem().and_then(|name| name.to_str()).map(String::from)
                } else {
                    None
                }
            })
            .collect();

        let max_width = databases.iter().map(|name| name.len()).max().unwrap_or(10);
        let box_width = max_width + 4; // Padding on sides

        println!("{}", "+".to_string() + &"-".repeat(box_width) + "+");

        println!("|{:^width$}|", "DATABASES", width = box_width);

        for db in databases {
            println!("|  {:<width$}  |", db, width = max_width);
        }

        println!("{}", "+".to_string() + &"-".repeat(box_width) + "+");

        Ok(())
    }

    fn list_tables(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let db_content = self.read_database()?;

        let tables: Vec<String> = db_content.keys().cloned().collect();

        let max_width = tables.iter().map(|name| name.len()).max().unwrap_or(10);
        let box_width = max_width + 4; // Padding on sides

        println!("{}", "+".to_string() + &"-".repeat(box_width) + "+");
        println!("|{:^width$}|", "TABLES", width = box_width);

        for table in tables {
            println!("|  {:<width$}  |", table, width = max_width);
        }

        println!("{}", "+".to_string() + &"-".repeat(box_width) + "+");

        Ok(())
    }


    fn read_database(&mut self) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
        let file_path = self.global_db_file.as_ref().ok_or("No database file set")?;
        
        if std::path::Path::new(file_path).exists() && std::fs::metadata(file_path)?.len() > 0 {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).map_err(Into::into)
        } else {
            Ok(Map::new())
        }
    }


    fn write_database(&self, db: Map<String, Value>) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.global_db_file.as_ref().ok_or("No database file set")?;
        let db_json = serde_json::to_string_pretty(&db)?;
        fs::write(file_path, db_json.as_bytes())?;
        Ok(())
    }
    fn expr_to_string(&self, expr: &NodeExpr) -> Result<String, &'static str> {
        match expr {
            NodeExpr::Identifier(NodeExprIdentifier { name }) => Ok(name.clone()),
        }
    }
}

