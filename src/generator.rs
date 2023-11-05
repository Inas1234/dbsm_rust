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

            }
        }

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

