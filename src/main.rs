use crate::parser::NodeStmt;

mod tokenizer;
mod parser;

fn main() {
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_contents = input.trim();

        if trimmed_contents == "exit" {
            println!("Exiting...");
            break;
        }

        if input == "exit" {
            println!("Exiting...");
            break;
        }
        let mut tokenizer = tokenizer::Tokenizer::new(input);
        let tokens: Vec<tokenizer::Token> = tokenizer.tokenize();
        let mut parser = parser::Parser::new(tokens);

        match parser.parse_stmt() {
            Some(stmt) => match stmt {
                NodeStmt::CreateTable(create_table_stmt) => {
                    // Verify the parsed table name and columns
                    println!("{}", create_table_stmt.table_name.name);
                    for expr in create_table_stmt.columns {
                        println!("{}", expr.name);
                    }
                }
                // Add other cases for different statements
                _ => println!("Parsed a different kind of statement."),
            },
            None => println!("Failed to parse the statement."),
        }

    }
}
