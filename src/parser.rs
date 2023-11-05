
use crate::tokenizer::{self, Token};


#[derive(Debug)]
pub struct NodeExprIdentifier{
    pub name: String,
}

#[derive(Debug)]
pub struct NodeStmtCreateTable{
    pub table_name: NodeExpr,
    pub columns: Vec<NodeExpr>,
} 

#[derive(Debug)]
pub enum NodeExpr{
    Identifier(NodeExprIdentifier)
}

#[derive(Debug)]
pub enum NodeStmt{
    CreateTable(NodeStmtCreateTable)
    
}

pub struct NodeProg{
   pub nodes: Vec<NodeStmt>
}

impl NodeProg {
    pub fn new() -> Self {
        NodeProg { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, node: NodeStmt) {
        self.nodes.push(node);
    }
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
                        println!("name: {}", name);
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


        let table_name = if let Some(ident) = self.parse_expression() {
            ident
        } else {
            return None;
        };

        if self.consume().map(|t| t.token) != Some(tokenizer::TokenType::LBRACE) {
            return None;
        }

        while let Some(column) = self.parse_expression() {
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

    pub fn parse_expression(&mut self) -> Option<NodeExpr> {

        if let Some(identifier) = self.parse_identifier() {
            return Some(NodeExpr::Identifier(identifier));
        }

        
        None
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


    pub fn parse_prog(&mut self) -> Option<NodeProg>{
        let mut prog = NodeProg::new();
        while self.index < self.tokens.len() {
            if let Some(stmt) = self.parse_stmt(){
                prog.add_node(stmt);
            }else {
                println!("ERRROR: there has been and error parsing the stmt");
                return None;
            }
        }

        if prog.nodes.is_empty(){
            println!("ERRROR: there has been and error parsing the prog, shits emoty");
            None
        }else{
            Some(prog)
        }
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
