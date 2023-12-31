
mod tokenizer;
mod parser;
mod generator;

fn main() {
    let mut generator = generator::Generator::new();
    loop {
        print!("mlinql> ");
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
        match parser.parse_prog() {
            Some(prog) => {
                match generator.generate(prog) {
                    Ok(_) => println!("Command executed."),
                    Err(e) => eprintln!("Failed to generate database: {}", e),
                }
            },
            None => eprintln!("Failed to parse program."),
        }
    }
}
