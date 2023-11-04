mod tokenizer;

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
                
        if input == "exit"{
            println!("Exiting...");
            break;
        }
        let mut tokenizer = tokenizer::Tokenizer::new(input);
        let tokens: Vec<tokenizer::Token> = tokenizer.tokenize();

        for token in tokens{
            match token.token {
                tokenizer::TokenType::CREATE => println!("CREATE TOKEN"),
                tokenizer::TokenType::TABLE => println!("TABLE TOKEN"),
                tokenizer::TokenType::IDENTIFIER => println!("IDENTIFIER TOKEN WITH VALUE: {:?}", token.value),
            }
        }
           
       }
}
