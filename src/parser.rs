pub mod ast;

use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::parser::ast::{AST, Node, Operator, VariableSpecifier};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    ast: Node,
    errors: Vec<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            position: 0,
            ast: Node::ProgramNode { children: Vec::new() },
            errors: Vec::new(),
        }
    }

    pub fn clean(&mut self) {
        self.tokens = Vec::new();
        self.ast = Node::ProgramNode { children: Vec::new() };
        self.errors = Vec::new();
        self.position = 0;
    }

    pub fn parse(&mut self, input: Vec<Token>) -> AST {
        self.clean();

        self.tokens = input;

        let mut ast = Node::ProgramNode { children: Vec::new() };

        while self.position < self.tokens.len() {
            if let Node::ProgramNode { children } = &mut ast {
                let token = self.peek();

                match token.token_type {
                    TokenType::IMPORT => {
                        let statement = match self.parse_import_statement() {
                            Ok(statement) => statement,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(statement);
                    }
                    TokenType::PROCESS => {
                        let process_node = match self.parse_process() {
                            Ok(process_node) => process_node,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(process_node);
                    }
                    TokenType::BLOCK => {
                        let block_node = match self.parse_block() {
                            Ok(block_node) => block_node,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(block_node);
                    }
                    TokenType::INPUT | TokenType::OUTPUT | TokenType::LET | TokenType::CONST => {
                        let variable_declaration_stmt = match self.parse_variable_declaration_stmt() {
                            Ok(variable_declaration_stmt) => variable_declaration_stmt,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(variable_declaration_stmt);
                    }
                    TokenType::BUFFER => {
                        let buffer_statement = match self.parse_buffer_declaration_stmt() {
                            Ok(buffer_statement) => buffer_statement,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(buffer_statement);
                    }
                    TokenType::ID => {
                        let statement = match self.parse_statement() {
                            Ok(statement) => statement,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(statement);
                    }
                    TokenType::EXPORT => {
                        let export_statement = match self.parse_export_declaration_stmt() {
                            Ok(export_statement) => export_statement,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(export_statement);
                    }
                    TokenType::CONNECT => {
                        let connect_statement = match self.parse_connect() {
                            Ok(connect_statement) => connect_statement,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(connect_statement);
                    }
                    TokenType::PARAM => {
                        let parameter_declaration_stmt = match self.parse_parameter_declaration_stmt() {
                            Ok(parameter_declaration_stmt) => parameter_declaration_stmt,
                            Err(e) => {
                                self.errors.push(e);

                                return AST {
                                    root: ast.clone(),
                                    errors: self.errors.clone(),
                                };
                            }
                        };

                        children.push(parameter_declaration_stmt);
                    }
                    TokenType::EOF => {
                        break;
                    }
                    _ => {
                        self.errors.push(format!("Unexpected token: {}", token.to_string()));

                        return AST {
                            root: ast.clone(),
                            errors: self.errors.clone(),
                        };
                    }
                }
            }
        }

        AST {
            root: ast.clone(),
            errors: Vec::new(),
        }
    }

    fn parse_import_statement(&mut self) -> Result<Node, String> {
        self.skip(TokenType::IMPORT)?;
        let id = match self.parse_id() {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        self.skip(TokenType::FROM)?;

        let path = self.consume();
        if let TokenType::STRING = path.token_type {
            let path = path.literal.clone();

            // Remove quotes
            let path = path[1..path.len() - 1].to_string();

            let node = Node::ImportStatement {
                id: Box::new(id),
                path,
            };

            self.skip(TokenType::SEMI)?;
            Ok(node)
        } else {
            Err(self.generic_error(&path, "Expected string literal"))
        }
    }

    fn parse_buffer_declaration_stmt(&mut self) -> Result<Node, String> {
        self.skip(TokenType::BUFFER)?;
        let id = match self.parse_id() {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        self.skip(TokenType::LSQUARE)?;
        let specifier = self.parse_number()?;
        self.skip(TokenType::RSQUARE)?;

        let token = self.peek();

        let node = match token.token_type {
            TokenType::SEMI => {
                Node::BufferDeclarationStmt {
                    id: Box::new(id),
                    initializer: Box::new(Node::Number(0.0)),
                    size: Box::new(specifier),
                }
            }
            TokenType::DEF => {
                self.skip(TokenType::DEF)?;
                let initializer = self.parse_buffer_initialization()?;
                Node::BufferDeclarationStmt {
                    id: Box::new(id),
                    initializer: Box::new(initializer),
                    size: Box::new(specifier),
                }
            }
            _ => {
                return Err(self.generic_error(&token, "';' or buffer initializer"));
            }
        };

        self.skip(TokenType::SEMI)?;

        Ok(node)
    }

    fn parse_buffer_initialization(&mut self) -> Result<Node, String> {
        self.skip(TokenType::BUFI)?;
        self.skip(TokenType::LCURLY)?;

        let mut buffer_initialization = Node::BufferInitializer {
            children: Vec::new(),
        };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::BufferInitializer { children } = &mut buffer_initialization {
                let statement = self.parse_statement()?;
                children.push(statement);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(buffer_initialization)
    }

    fn parse_parameter_declaration_stmt(&mut self) -> Result<Node, String> {
        self.skip(TokenType::PARAM)?;
        let id = match self.parse_id() {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        self.skip(TokenType::LCURLY)?;

        let mut parameter_declaration_stmt = Node::ParameterDeclarationStmt {
            id: Box::new(id),
            fields: Vec::new(),
        };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ParameterDeclarationStmt { id: _, fields } = &mut parameter_declaration_stmt {
                let field = match self.parse_parameter_declaration_field() {
                    Ok(field) => field,
                    Err(e) => return Err(e),
                };
                fields.push(field);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(parameter_declaration_stmt)
    }

    fn parse_parameter_declaration_field(&mut self) -> Result<Node, String> {
        let id = match self.parse_id() {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        self.skip(TokenType::COLON)?;
        let specifier = self.parse_number()?;

        let specifier = match specifier {
            Node::Number(n) => n,
            _ => {
                return Err(self.generic_error(&self.tokens[self.position - 1], "Expected number"));
            }
        };

        self.skip(TokenType::SEMI)?;

        Ok(Node::ParameterDeclarationField {
            id: Box::new(id),
            specifier,
        })
    }

    fn parse_connect(&mut self) -> Result<Node, String> {
        self.skip(TokenType::CONNECT)?;
        self.skip(TokenType::LCURLY)?;

        let mut connect = Node::ConnectNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ConnectNode { children } = &mut connect {
                let child = self.parse_connect_statement()?;
                children.push(child);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(connect)
    }

    fn parse_connect_statement(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                let lhs = self.parse_connection_member()?;

                self.skip(TokenType::CABLE)?;
                let rhs = self.parse_right_connection_member()?;

                println!("LHS: {:?}", lhs);
                println!("RHS: {:?}", rhs);

                self.skip(TokenType::SEMI)?;

                Ok(Node::ConnectStmt {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            _ => {
                Err(self.generic_error(&token, "identifier"))
            }
        }
    }

    fn parse_right_connection_member(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                self.parse_connection_member()
            }
            TokenType::OUTPUTS => {
                self.parse_outputs_stmt()
            }
            _ => {
                Err(self.generic_error(&token, "identifier or outputs specifier"))
            }
        }
    }

    fn parse_outputs_stmt(&mut self) -> Result<Node, String> {
        self.skip(TokenType::OUTPUTS)?;

        let token = self.peek();

        if token.token_type != TokenType::LSQUARE {
            return Ok(Node::OutputsStmt);
        }

        self.skip(TokenType::LSQUARE)?;
        let specifier = self.parse_number()?;
        self.skip(TokenType::RSQUARE)?;

        if let Node::Number(number) = specifier {
            return Ok(Node::OutputsNumberedStmt(number as i32));
        }

        Err(self.generic_error(&token, "number"))
    }

    fn parse_connection_member(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();

                if next_token.token_type != TokenType::DOT {
                    return self.parse_id();
                }

                self.parse_member_expr()
            }
            _ => {
                Err(self.generic_error(&token, "identifier"))
            }
        }
    }

    fn parse_export_declaration_stmt(&mut self) -> Result<Node, String> {
        self.skip(TokenType::EXPORT)?;

        let token = self.peek();

        let declaration = match token.token_type {
            TokenType::INPUT | TokenType::OUTPUT | TokenType::LET | TokenType::CONST => {
                self.parse_variable_declaration_stmt()?
            }
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();

                match next_token.token_type {
                    TokenType::LPAREN => {
                        self.parse_function_declaration_stmt()?
                    }
                    _ => {
                        Err(self.generic_error(&next_token, "function declaration"))?
                    }
                }
            }
            _ => {
                Err(self.generic_error(&token, "variable declaration or function declaration"))?
            }
        };

        Ok(Node::ExportDeclarationStmt {
            declaration: Box::new(declaration),
        })
    }

    fn parse_statement(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();

                let node = match next_token.token_type {
                    TokenType::LPAREN => {
                        self.parse_function_declaration_stmt()
                    }
                    TokenType::DEF => {
                        self.parse_assignment_expression()
                    }
                    _ => {
                        Err(self.generic_error(&next_token, "function declaration or assignment expression"))?
                    }
                };

                node
            }
            TokenType::LET | TokenType::CONST => {
                self.parse_variable_declaration_stmt()
            }
            TokenType::RETURN => {
                self.parse_return_stmt()
            }
            _ => {
                Err(self.generic_error(&token, "statement"))?
            }
        }
    }

    fn parse_return_stmt(&mut self) -> Result<Node, String> {
        self.skip(TokenType::RETURN)?;
        let expr = self.parse_expression()?;
        self.skip(TokenType::SEMI)?;

        Ok(Node::ReturnStmt {
            child: Box::new(expr),
        })
    }

    fn parse_function_declaration_stmt(&mut self) -> Result<Node, String> {
        let id = self.parse_id()?;
        self.skip(TokenType::LPAREN)?;
        let params = self.parse_params()?;
        self.skip(TokenType::RPAREN)?;
        let body = self.parse_function_body()?;

        Ok(Node::FunctionDeclarationStmt {
            id: Box::new(id),
            params,
            body: Box::new(body),
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Node>, String> {
        let mut params = Vec::new();

        while self.tokens[self.position].token_type != TokenType::RPAREN {
            let id = self.parse_id()?;
            params.push(id);

            if self.tokens[self.position].token_type == TokenType::COMMA {
                self.skip(TokenType::COMMA)?;
            }
        }

        Ok(params)
    }

    fn parse_process(&mut self) -> Result<Node, String> {
        // Should skip {
        self.skip(TokenType::PROCESS)?;
        self.skip(TokenType::LCURLY)?;

        let mut process = Node::ProcessNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ProcessNode { children } = &mut process {
                let child = self.parse_statement()?;
                children.push(child);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(process)
    }

    fn parse_block(&mut self) -> Result<Node, String> {
        // Should skip {
        self.skip(TokenType::BLOCK)?;
        self.skip(TokenType::LCURLY)?;

        let mut process = Node::BlockNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::BlockNode { children } = &mut process {
                children.push(self.parse_statement()?);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(process)
    }

    fn parse_function_body(&mut self) -> Result<Node, String> {
        // Should skip {
        self.skip(TokenType::LCURLY)?;

        let mut process = Node::FunctionBody { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::FunctionBody { children } = &mut process {
                let child = self.parse_statement()?;
                children.push(child);
            }
        }

        self.skip(TokenType::RCURLY)?;

        Ok(process)
    }

    fn parse_variable_specifier(&mut self) -> Result<VariableSpecifier, String> {
        let token = self.consume();

        let result = match token.token_type {
            TokenType::INPUT => {
                VariableSpecifier::Input
            }
            TokenType::OUTPUT => {
                VariableSpecifier::Output
            }
            TokenType::LET => {
                VariableSpecifier::Let
            }
            TokenType::CONST => {
                VariableSpecifier::Const
            }
            _ => {
                Err(self.generic_error(&token, "variable specifier"))?
            }
        };

        Ok(result)
    }

    fn parse_variable_declaration_stmt(&mut self) -> Result<Node, String> {
        let specifier = self.parse_variable_specifier()?;
        let id = self.parse_id()?;
        self.skip(TokenType::DEF)?;
        let initializer = self.parse_expression()?;
        self.skip(TokenType::SEMI)?;

        Ok(Node::VariableDeclarationStmt {
            id: Box::new(id),
            initializer: Box::new(initializer),
            specifier,
        })
    }

    fn parse_assignment_expression(&mut self) -> Result<Node, String> {
        let id = self.parse_id()?;
        self.skip(TokenType::DEF)?;

        let expr = self.parse_expression()?;

        println!("id: {:?}, expr: {:?}", id, expr);

        self.skip(TokenType::SEMI)?;

        Ok(Node::AssignmentExpr {
            lhs: Box::new(id),
            rhs: Box::new(expr),
        })
    }

    fn parse_expression(&mut self) -> Result<Node, String> {
        /*
        Expression is defined as:
        expr -> infix_expr | unary_expr
        infix_expr -> expr op expr | expr LPAR params RPAR
        unary_expr -> op expr
         */

        let token = self.peek();

        match token.token_type {
            TokenType::PLUS | TokenType::MINUS => {
                self.parse_unary_expr()
            }
            _ => {
                self.parse_infix_expr()
            }
        }
    }

    fn parse_unary_expr(&mut self) -> Result<Node, String> {
        let token = self.consume();

        match token.token_type {
            TokenType::PLUS => {
                let expr = self.parse_expression()?;
                Ok(Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(expr),
                })
            }
            TokenType::MINUS => {
                let expr = self.parse_expression()?;
                Ok(Node::UnaryExpr {
                    op: Operator::Minus,
                    child: Box::new(expr),
                })
            }
            _ => {
                Err(self.generic_error(&token, "plus or minus"))
            }
        }
    }

    fn parse_infix_expr(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::LPAREN | TokenType::NUMBER | TokenType::ID => {
                self.parse_binary_expr()
            }
            _ => {
                Err(self.generic_error(&token, "expression"))
            }
        }
    }

    fn parse_member_expr(&mut self) -> Result<Node, String> {
        let id = self.parse_id()?;
        self.skip(TokenType::DOT)?;
        let member = self.parse_id()?;

        Ok(Node::MemberExpr {
            object: Box::new(id),
            property: Box::new(member),
        })
    }

    fn parse_binary_expr(&mut self) -> Result<Node, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Node, String> {
        let mut lhs = self.parse_add_sub()?;

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::GT | TokenType::LT | TokenType::GE | TokenType::LE => {
                    let op = self.parse_operator()?;
                    let rhs = self.parse_add_sub()?;
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    fn parse_add_sub(&mut self) -> Result<Node, String> {
        let mut lhs = self.parse_mul_div()?;

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::PLUS | TokenType::MINUS => {
                    let op = self.parse_operator()?;
                    let rhs = self.parse_mul_div()?;
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    fn parse_mul_div(&mut self) -> Result<Node, String> {
        let mut lhs = self.parse_primitive()?;

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::MUL | TokenType::DIV => {
                    let op = self.parse_operator()?;
                    let rhs = self.parse_primitive()?;
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(lhs)
    }


    fn parse_primitive(&mut self) -> Result<Node, String> {
        let token = self.peek();

        match token.token_type {
            TokenType::LPAREN => {
                self.skip(TokenType::LPAREN)?;
                let expr = self.parse_expression()?;
                self.skip(TokenType::RPAREN)?;
                Ok(expr)
            }
            TokenType::MINUS => {
                self.parse_unary_expr()
            }
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();
                match next_token.token_type {
                    TokenType::LPAREN => {
                        self.parse_fn_call()
                    }
                    TokenType::DOT => {
                        self.parse_member_expr()
                    }
                    _ => {
                        self.parse_id()
                    }
                }
            }
            TokenType::NUMBER => {
                self.parse_number()
            }
            _ => {
                Err(self.generic_error(&token, "(, -, id, number)"))
            }
        }
    }

    fn parse_number(&mut self) -> Result<Node, String> {
        let token = self.consume();

        match token.token_type {
            TokenType::NUMBER => {
                Ok(Node::Number(token.literal.parse::<f64>().unwrap()))
            }
            _ => {
                Err(self.generic_error(&token, "number"))
            }
        }
    }

    fn parse_fn_call(&mut self) -> Result<Node, String> {
        let id = self.parse_id()?;
        self.skip(TokenType::LPAREN)?;
        let args = self.parse_arguments()?;
        self.skip(TokenType::RPAREN)?;

        Ok(Node::FnCallExpr {
            id: Box::new(id),
            args,
        })
    }

    fn parse_arguments(&mut self) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::COMMA => {
                    self.skip(TokenType::COMMA)?;
                }
                TokenType::RPAREN => {
                    break;
                }
                _ => {
                    args.push(self.parse_expression()?);
                }
            }
        }

        Ok(args)
    }

    fn parse_operator(&mut self) -> Result<Operator, String> {
        let tok = self.consume();

        let result = match tok.token_type {
            TokenType::PLUS => Operator::Plus,
            TokenType::MINUS => Operator::Minus,
            TokenType::MUL => Operator::Mul,
            TokenType::DIV => Operator::Div,
            TokenType::EQ => Operator::Eq,
            TokenType::GT => Operator::Gt,
            TokenType::LT => Operator::Lt,
            TokenType::GE => Operator::Ge,
            TokenType::LE => Operator::Le,
            _ => {
                return Err(self.generic_error(&tok, "operator"));
            }
        };

        Ok(result)
    }

    fn parse_id(&mut self) -> Result<Node, String> {
        let token = self.consume();

        match token.token_type {
            TokenType::ID => {
                Ok(Node::Identifier(token.literal))
            }
            _ => {
                Err(self.generic_error(&token, "identifier"))
            }
        }
    }

    fn generic_error(&self, token: &Token, expected: &str) -> String {
        format!("Unexpected token: {}, expected {}", token.to_string(), expected)
    }

    fn skip(&mut self, token_type: TokenType) -> Result<(), String> {
        if self.tokens[self.position].token_type == token_type {
            self.position += 1;
            Ok(())
        } else {
            println!("PARSED: {:?}", self.ast);
            println!("TOKENS: {:?}", self.tokens);
            println!("POSITION: {:?}", self.position);

            Err(format!("Unexpected token: {}, expected {:?}", self.tokens[self.position].to_string(), token_type))
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::{Position, Token};
    use crate::lexer::token_type::TokenType;
    use crate::parser::{AST, Node, Parser};

    #[test]
    fn test_parser_lotion() {
        let tokens = vec![
            Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }),
            Token::new(TokenType::LCURLY, "{".to_string(), Position { start: 5, end: 6, column: 5, line: 0 }),
            Token::new(TokenType::RCURLY, "}".to_string(), Position { start: 6, end: 7, column: 6, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }),
        ];

        let mut parser = Parser::new();
        let ast = parser.parse(tokens);

        assert_eq!(ast, AST {
            root: Node::ProgramNode {
                children: vec![
                    Node::BlockNode {
                        children: vec![],
                    },
                ],
            },
            errors: vec![],
        });
    }

    #[test]
    fn test_consume() {
        let tokens = vec![
            Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }),
            Token::new(TokenType::LCURLY, "{".to_string(), Position { start: 5, end: 6, column: 5, line: 0 }),
            Token::new(TokenType::RCURLY, "}".to_string(), Position { start: 6, end: 7, column: 6, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let token = parser.consume();
        assert_eq!(token, Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }));
        assert_eq!(parser.position, 1);

        let token = parser.consume();
        assert_eq!(token, Token::new(TokenType::LCURLY, "{".to_string(), Position { start: 5, end: 6, column: 5, line: 0 }));
        assert_eq!(parser.position, 2);

        let token = parser.consume();
        assert_eq!(token, Token::new(TokenType::RCURLY, "}".to_string(), Position { start: 6, end: 7, column: 6, line: 0 }));
        assert_eq!(parser.position, 3);

        let token = parser.consume();
        assert_eq!(token, Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }));
        assert_eq!(parser.position, 4);
    }

    #[test]
    fn test_peek() {
        let tokens = vec![
            Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }),
            Token::new(TokenType::LCURLY, "{".to_string(), Position { start: 5, end: 6, column: 5, line: 0 }),
            Token::new(TokenType::RCURLY, "}".to_string(), Position { start: 6, end: 7, column: 6, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let token = parser.peek();
        assert_eq!(token, Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }));
        assert_eq!(parser.position, 0);

        let token = parser.peek();
        assert_eq!(token, Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }));
        assert_eq!(parser.position, 0);
    }

    #[test]
    fn test_skip() {
        let tokens = vec![
            Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }),
            Token::new(TokenType::LCURLY, "{".to_string(), Position { start: 5, end: 6, column: 5, line: 0 }),
            Token::new(TokenType::RCURLY, "}".to_string(), Position { start: 6, end: 7, column: 6, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let result = parser.skip(TokenType::BLOCK);
        assert_eq!(result, Ok(()));
        assert_eq!(parser.position, 1);

        let result = parser.skip(TokenType::LCURLY);
        assert_eq!(result, Ok(()));
        assert_eq!(parser.position, 2);

        let result = parser.skip(TokenType::RCURLY);
        assert_eq!(result, Ok(()));
        assert_eq!(parser.position, 3);

        let result = parser.skip(TokenType::UNKNOWN); // Should be EOF, but let's test the error message
        assert_eq!(result, Err("Unexpected token: <EOF: [Position { start: 7, end: 7, line: 0, column: 7 }]>, expected UNKNOWN".to_string()));
        assert_eq!(parser.position, 3);
    }

    #[test]
    fn test_generic_error() {
        let tokens = vec![
            Token::new(TokenType::BLOCK, "block".to_string(), Position { start: 0, end: 5, column: 0, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 7, end: 7, column: 7, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let token = parser.consume();

        let result = parser.generic_error(&token, "Test error");

        assert_eq!(result, "Unexpected token: <BLOCK:block [Position { start: 0, end: 5, line: 0, column: 0 }]>, expected Test error");
    }

    #[test]
    fn test_parse_id() {
        let tokens = vec![
            Token::new(TokenType::ID, "id".to_string(), Position { start: 0, end: 2, column: 0, line: 0 }),
            Token::new(TokenType::EOF, "".to_string(), Position { start: 2, end: 2, column: 2, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let result = parser.parse_id();

        assert_eq!(result, Ok(Node::Identifier("id".to_string())));
    }

    #[test]
    fn test_parse_id_error() {
        let tokens = vec![
            Token::new(TokenType::EOF, "".to_string(), Position { start: 0, end: 0, column: 0, line: 0 }),
        ];

        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser.position = 0;

        let result = parser.parse_id();

        assert_eq!(result, Err("Unexpected token: <EOF: [Position { start: 0, end: 0, line: 0, column: 0 }]>, expected identifier".to_string()));
    }
}
