
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    CREATE,
    TABLE,
    IDENTIFIER,
    LBRACE,
    RBRACE,
    COMMA
}

pub struct Token {
    pub token: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    contents: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(contents: String) -> Self {
        Tokenizer { contents, index: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buffer = String::new();

        while let Some(c) = self.peek(0) {
            if c.is_alphabetic() {
                buffer.push(self.consume());
                while let Some(next_c) = self.peek(0) {
                    if next_c.is_alphanumeric() {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }

                match buffer.as_str() {
                    "CREATE" => tokens.push(Token { token: TokenType::CREATE, value: None }),
                    "TABLE" => tokens.push(Token { token: TokenType::TABLE, value: None }),
                    _ => tokens.push(Token { token: TokenType::IDENTIFIER, value: Some(buffer.clone()) }),
                }
                buffer.clear();
            }
            else if c == ','{
                self.consume();
                tokens.push(Token { token: TokenType::COMMA, value: None });
                continue;
            } 
            else if c == '{'{
                self.consume();
                tokens.push(Token { token: TokenType::LBRACE, value: None });
                continue;
            }
            else if c == '}'{
                self.consume();
                tokens.push(Token { token: TokenType::RBRACE, value: None });
                continue;
            }
            else {
                self.consume(); 
            }
        }

        self.index = 0;

        tokens
    }

    fn peek(&self, ahead: usize) -> Option<char> {
        let index = self.index + ahead;
        if index >= self.contents.len() {
            None
        } else {
            self.contents.chars().nth(index)
        }
    }

    fn consume(&mut self) -> char {
        if self.index < self.contents.len() {
            let c = self.contents[self.index..].chars().next().unwrap();
            self.index += c.len_utf8();
            c
        } else {
            '\0' 
        }
    }
}
