
use crate::tokenizer::{self, Token};


pub struct NodeExprIdentifier{
    pub name: String,
}

pub struct NodeStmtCreateTable{
    pub table_name: NodeExprIdentifier,
    pub columns: Vec<NodeExprIdentifier>,
} 

pub enum NodeExpr{
    Identifier(NodeExprIdentifier)
}

pub enum NodeStmt{
    CreateTable(NodeStmtCreateTable)
    
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(m_tokens: Vec<Token>) -> Self{
        Parser { tokens: m_tokens, index: 0 }
    }


    pub fn parse_identifier(&mut self) -> Option<NodeExprIdentifier>{
        if let Some(token) = self.consume() {
            match token.token {
                tokenizer::TokenType::IDENTIFIER => {
                    if let Some(name) = &token.value {
                        return  Some(NodeExprIdentifier { name: name.clone() });
                    }
                }
                _ => {}
            }
        }
        None
    }
    pub fn parse_create_table(&mut self) -> Option<NodeStmtCreateTable> {
        let mut columns = Vec::new();

        self.consume();

        if self.consume().map(|t| t.token) != Some(tokenizer::TokenType::TABLE) {
            return None;
        }


        let table_name = if let Some(ident) = self.parse_identifier() {
            ident
        } else {
            return None;
        };

        if self.consume().map(|t| t.token) != Some(tokenizer::TokenType::LBRACE) {
            return None;
        }

        while let Some(column) = self.parse_identifier() {
            columns.push(column);
            if self.peek(0).map(|t| t.token) == Some(tokenizer::TokenType::COMMA) {
                self.consume();
            } else {
                break;
            }
        }

        if self.consume().map(|t| t.token) != Some(tokenizer::TokenType::RBRACE) {
            return None;
        }

        Some(NodeStmtCreateTable {
            table_name,
            columns,
        })
    }


    pub fn parse_stmt(&mut self) -> Option<NodeStmt> {
        if let Some(token) = self.peek(0) {
            match token.token {
                tokenizer::TokenType::CREATE => {
                    if self.peek(1).map(|t| t.token) == Some(tokenizer::TokenType::TABLE) {
                        if let Some(create_table_stmt) = self.parse_create_table() {
                            return Some(NodeStmt::CreateTable(create_table_stmt));
                        }
                    }
                },
                _ => {}
            }
        }
        None
    }


    fn consume(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            let token = &self.tokens[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self, ahead: usize) -> Option<&Token> {
        if self.index + ahead >= self.tokens.len(){ 
            None
        }
        else {
            Some(&self.tokens[self.index + ahead])
        }
    }

}
