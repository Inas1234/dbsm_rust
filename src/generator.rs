use std::fs::{ self};
use std::io::BufReader;
use serde_json::{Value, Map};
use crate::parser::{NodeStmt, NodeExprIdentifier, NodeExpr, NodeProg};



pub(crate) struct Generator{
    output_file: String
}

impl Generator{
      pub fn new(output_file: String) -> Self {
        Generator { output_file }
    }

    pub fn generate(&self, prog: NodeProg) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = self.read_database()?;

        for stmt in prog.nodes {
            match stmt {
                NodeStmt::CreateTable(stmt) => {
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
                }
            }
        }

        self.write_database(db)?;

        Ok(())
    }

    fn expr_to_string(&self, expr: &NodeExpr) -> Result<String, &'static str> {
        match expr {
            NodeExpr::Identifier(NodeExprIdentifier { name }) => Ok(name.clone()),
        }
    }

    //...
    fn read_database(&self) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
        let file = match fs::File::open(&self.output_file) {
            Ok(file) => file,
            Err(_) => {
                let file = fs::File::create(&self.output_file)?;
                serde_json::to_writer(&file, &serde_json::json!({}))?;
                return Ok(Map::new());
            }
        };

        let reader = BufReader::new(file);

        if reader.buffer().is_empty() {
            Ok(Map::new())
        } else {
            serde_json::from_reader(reader).map_err(Into::into)
        }
    }

    fn write_database(&self, db: Map<String, Value>) -> Result<(), Box<dyn std::error::Error>> {
        let db_json = serde_json::to_string_pretty(&db)?;
        fs::write(&self.output_file, db_json.as_bytes()).map_err(Into::into)
    }
}