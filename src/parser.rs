use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Debug, Clone)]
pub enum VariableSpecifier {
    Let,
    Const,
    Input,
    Output,
    Buffer,
}

#[derive(Debug, Clone)]
pub enum Node {
    ProgramNode {
        children: Vec<Node>,
    },
    ProcessNode {
        children: Vec<Node>,
    },
    BlockNode {
        children: Vec<Node>,
    },
    ConnectNode {
        children: Vec<Node>,
    },
    FunctionBody {
        children: Vec<Node>,
    },
    Identifier(String),
    ExpressionStmt {
        child: Box<Node>,
    },
    AssignmentExpr {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ConnectStmt {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ReturnStmt {
        child: Box<Node>,
    },
    VariableDeclarationStmt {
        id: Box<Node>,
        initializer: Box<Node>,
        specifier: VariableSpecifier,
    },
    FunctionDeclarationStmt {
        id: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    MemberExpr {
        object: Box<Node>,
        property: Box<Node>,
    },
    ExportDeclarationStmt {
        declaration: Box<Node>,
    },

    ParameterDeclarationStmt {
        id: Box<Node>,
        fields: Vec<Node>,
    },

    ParameterDeclarationField {
        id: Box<Node>,
        specifier: f64,
    },

    FnCallExpr {
        id: Box<Node>,
        args: Vec<Node>,
    },

    Number(f64),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OutputsStmt,
    OutputsNumberedStmt(i32),
    BufferDeclarationStmt {
        id: Box<Node>,
        size: Box<Node>,
        initializer: Box<Node>,
    },
    BufferInitializer {
        children: Vec<Node>,
    },
    ImportStatement {
        id: Box<Node>,
        path: String,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    ast: Node,
}

#[derive(Debug, Clone)]
pub struct AST {
    pub root: Node,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        Parser {
            tokens: input,
            position: 0,
            ast: Node::ProgramNode { children: Vec::new() },
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut ast = Node::ProgramNode { children: Vec::new() };

        while self.position < self.tokens.len() {
            if let Node::ProgramNode { children } = &mut ast {
                let token = self.peek();

                match token.token_type {
                    TokenType::IMPORT => {
                        children.push(self.parse_import_statement())
                    }
                    TokenType::PROCESS => {
                        children.push(self.parse_process())
                    }
                    TokenType::BLOCK => {
                        children.push(self.parse_block())
                    }
                    TokenType::INPUT | TokenType::OUTPUT | TokenType::LET | TokenType::CONST => {
                        children.push(self.parse_variable_declaration_stmt())
                    }
                    TokenType::BUFFER => {
                        children.push(self.parse_buffer_declaration_stmt())
                    }
                    TokenType::ID => {
                        children.push(self.parse_statement())
                    }
                    TokenType::EXPORT => {
                        children.push(self.parse_export_declaration_stmt())
                    }
                    TokenType::CONNECT => {
                        children.push(self.parse_connect())
                    }
                    TokenType::PARAM => {
                        children.push(self.parse_parameter_declaration_stmt())
                    }
                    TokenType::EOF => {
                        break;
                    }
                    _ => {
                        panic!("Unexpected token: {}", token.to_string());
                    }
                }
            }
        }

        AST {
            root: ast.clone(),
            errors: Vec::new(),
        }
    }

    fn parse_import_statement(&mut self) -> Node {
        self.skip(TokenType::IMPORT);
        let id = self.parse_id();
        self.skip(TokenType::FROM);

        let path = self.consume();
        if let TokenType::STRING = path.token_type {
            let path = path.literal.clone();

            // Remove quotes
            let path = path[1..path.len() - 1].to_string();

            let node = Node::ImportStatement {
                id: Box::new(id),
                path,
            };

            self.skip(TokenType::SEMI);
            node
        } else {
            panic!("Expected string literal");
        }


    }

    fn parse_buffer_declaration_stmt(&mut self) -> Node {
        self.skip(TokenType::BUFFER);
        let id = self.parse_id();
        self.skip(TokenType::LSQUARE);
        let specifier = self.parse_number();
        self.skip(TokenType::RSQUARE);

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
                self.skip(TokenType::DEF);
                let initializer = self.parse_buffer_initialization();
                Node::BufferDeclarationStmt {
                    id: Box::new(id),
                    initializer: Box::new(initializer),
                    size: Box::new(specifier),
                }
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        };

        self.skip(TokenType::SEMI);

        node
    }

    fn parse_buffer_initialization(&mut self) -> Node {
        self.skip(TokenType::BUFI);
        self.skip(TokenType::LCURLY);

        let mut buffer_initialization = Node::BufferInitializer {
            children: Vec::new(),
        };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::BufferInitializer { children } = &mut buffer_initialization {
                children.push(self.parse_statement());
            }
        }

        self.skip(TokenType::RCURLY);

        buffer_initialization
    }

    fn parse_parameter_declaration_stmt(&mut self) -> Node {
        self.skip(TokenType::PARAM);
        let id = self.parse_id();
        self.skip(TokenType::LCURLY);

        let mut parameter_declaration_stmt = Node::ParameterDeclarationStmt {
            id: Box::new(id),
            fields: Vec::new(),
        };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ParameterDeclarationStmt { id, fields } = &mut parameter_declaration_stmt {
                fields.push(self.parse_parameter_declaration_field());
            }
        }

        self.skip(TokenType::RCURLY);

        parameter_declaration_stmt
    }

    fn parse_parameter_declaration_field(&mut self) -> Node {
        let id = self.parse_id();
        self.skip(TokenType::COLON);
        let specifier = self.parse_number();

        let specifier = match specifier {
            Node::Number(n) => n,
            _ => panic!("Expected number")
        };

        self.skip(TokenType::SEMI);

        Node::ParameterDeclarationField {
            id: Box::new(id),
            specifier
        }
    }

    fn parse_connect(&mut self) -> Node {
        self.skip(TokenType::CONNECT);
        self.skip(TokenType::LCURLY);

        let mut connect = Node::ConnectNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ConnectNode { children } = &mut connect {
                children.push(self.parse_connect_statement());
            }
        }

        self.skip(TokenType::RCURLY);

        connect
    }

    fn parse_connect_statement(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                let lhs = self.parse_connection_member();
                self.skip(TokenType::CABLE);
                let rhs = self.parse_right_connection_member();

                println!("LHS: {:?}", lhs);
                println!("RHS: {:?}", rhs);

                self.skip(TokenType::SEMI);

                Node::ConnectStmt {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_right_connection_member(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                self.parse_connection_member()
            }
            TokenType::OUTPUTS => {
                self.parse_outputs_stmt()
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_outputs_stmt(&mut self) -> Node {
        self.skip(TokenType::OUTPUTS);

        let token = self.peek();

        if token.token_type != TokenType::LSQUARE {
            return Node::OutputsStmt;
        }

        self.skip(TokenType::LSQUARE);
        let specifier = self.parse_number();
        self.skip(TokenType::RSQUARE);

        if let Node::Number(number) = specifier {
            return Node::OutputsNumberedStmt(number as i32);
        }

        panic!("Unexpected token: {}", token.to_string());
    }

    fn parse_connection_member(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();

                if next_token.token_type != TokenType::DOT {
                    return self.parse_id();
                }

                let member = self.parse_member_expr();

                member
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_export_declaration_stmt(&mut self) -> Node {
        self.skip(TokenType::EXPORT);

        let token = self.peek();

        let declaration = match token.token_type {
            TokenType::INPUT | TokenType::OUTPUT | TokenType::LET | TokenType::CONST => {
                self.parse_variable_declaration_stmt()
            }
            TokenType::ID => {
                let next_token = self.tokens[self.position + 1].clone();

                match next_token.token_type {
                    TokenType::LPAREN => {
                        self.parse_function_declaration_stmt()
                    }
                    _ => {
                        panic!("Unexpected token: {}", next_token.to_string());
                    }
                }
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        };

        Node::ExportDeclarationStmt {
            declaration: Box::new(declaration),
        }
    }

    fn parse_statement(&mut self) -> Node {
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
                        panic!("Unexpected token: {}", next_token.to_string());
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
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_return_stmt(&mut self) -> Node {
        self.skip(TokenType::RETURN);
        let expr = self.parse_expression();
        self.skip(TokenType::SEMI);

        Node::ReturnStmt {
            child: Box::new(expr),
        }
    }

    fn parse_function_declaration_stmt(&mut self) -> Node {
        let id = self.parse_id();
        self.skip(TokenType::LPAREN);
        let params = self.parse_params();
        self.skip(TokenType::RPAREN);
        let body = self.parse_function_body();

        Node::FunctionDeclarationStmt {
            id: Box::new(id),
            params,
            body: Box::new(body),
        }
    }

    fn parse_params(&mut self) -> Vec<Node> {
        let mut params = Vec::new();

        while self.tokens[self.position].token_type != TokenType::RPAREN {
            params.push(self.parse_id());

            if self.tokens[self.position].token_type == TokenType::COMMA {
                self.skip(TokenType::COMMA);
            }
        }

        params
    }

    fn parse_process(&mut self) -> Node {
        // Should skip {
        self.skip(TokenType::PROCESS);
        self.skip(TokenType::LCURLY);

        let mut process = Node::ProcessNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::ProcessNode { children } = &mut process {
                children.push(self.parse_statement());
            }
        }

        self.skip(TokenType::RCURLY);

        process
    }

    fn parse_block(&mut self) -> Node {
        // Should skip {
        self.skip(TokenType::BLOCK);
        self.skip(TokenType::LCURLY);

        let mut process = Node::BlockNode { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::BlockNode { children } = &mut process {
                children.push(self.parse_statement());
            }
        }

        self.skip(TokenType::RCURLY);

        process
    }

    fn parse_function_body(&mut self) -> Node {
        // Should skip {
        self.skip(TokenType::LCURLY);

        let mut process = Node::FunctionBody { children: Vec::new() };

        while self.tokens[self.position].token_type != TokenType::RCURLY {
            if let Node::FunctionBody { children } = &mut process {
                children.push(self.parse_statement());
            }
        }

        self.skip(TokenType::RCURLY);

        process
    }

    fn parse_variable_specifier(&mut self) -> VariableSpecifier {
        let token = self.consume();

        match token.token_type {
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
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_variable_declaration_stmt(&mut self) -> Node {
        let specifier = self.parse_variable_specifier();
        let id = self.parse_id();
        self.skip(TokenType::DEF);
        let initializer = self.parse_expression();
        self.skip(TokenType::SEMI);

        Node::VariableDeclarationStmt {
            id: Box::new(id),
            initializer: Box::new(initializer),
            specifier,
        }
    }

    fn parse_assignment_expression(&mut self) -> Node {
        let id = self.parse_id();
        self.skip(TokenType::DEF);

        let expr = self.parse_expression();

        println!("id: {:?}, expr: {:?}", id, expr);

        self.skip(TokenType::SEMI);

        Node::AssignmentExpr {
            lhs: Box::new(id),
            rhs: Box::new(expr),
        }
    }

    fn parse_expression(&mut self) -> Node {
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

    fn parse_unary_expr(&mut self) -> Node {
        let token = self.consume();

        match token.token_type {
            TokenType::PLUS => {
                Node::UnaryExpr {
                    op: Operator::Plus,
                    child: Box::new(self.parse_expression()),
                }
            }
            TokenType::MINUS => {
                Node::UnaryExpr {
                    op: Operator::Minus,
                    child: Box::new(self.parse_expression()),
                }
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_infix_expr(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::LPAREN | TokenType::NUMBER | TokenType::ID => {
                self.parse_binary_expr()
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_id_expr(&mut self) -> Node {
        // let id = self.peek();
        let token = self.tokens[self.position + 1].clone();

        match token.token_type {
            TokenType::LPAREN => {
                self.parse_fn_call()
            }
            TokenType::DOT => {
                self.parse_member_expr()
            }
            _ => {
                self.parse_binary_expr()
            }
        }
    }

    fn parse_member_expr(&mut self) -> Node {
        let id = self.parse_id();
        self.skip(TokenType::DOT);
        let member = self.parse_id();

        Node::MemberExpr {
            object: Box::new(id),
            property: Box::new(member),
        }
    }

    fn parse_binary_expr(&mut self) -> Node {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Node {
        let mut lhs = self.parse_add_sub();

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::GT | TokenType::LT | TokenType::GE | TokenType::LE => {
                    let op = self.parse_operator();
                    let rhs = self.parse_add_sub();
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        lhs
    }

    fn parse_add_sub(&mut self) -> Node {
        let mut lhs = self.parse_mul_div();

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::PLUS | TokenType::MINUS => {
                    let op = self.parse_operator();
                    let rhs = self.parse_mul_div();
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        lhs
    }

    fn parse_mul_div(&mut self) -> Node {
        let mut lhs = self.parse_primitive();

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::MUL | TokenType::DIV => {
                    let op = self.parse_operator();
                    let rhs = self.parse_primitive();
                    lhs = Node::BinaryExpr {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        lhs
    }


    fn parse_primitive(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::LPAREN => {
                self.skip(TokenType::LPAREN);
                let expr = self.parse_expression();
                self.skip(TokenType::RPAREN);
                expr
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
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_number(&mut self) -> Node {
        let token = self.consume();

        match token.token_type {
            TokenType::NUMBER => {
                Node::Number(token.literal.parse::<f64>().unwrap())
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn parse_fn_call(&mut self) -> Node {
        let id = self.parse_id();
        self.skip(TokenType::LPAREN);
        let args = self.parse_arguments();

        self.skip_until(TokenType::RPAREN);
        self.skip(TokenType::RPAREN);

        Node::FnCallExpr {
            id: Box::new(id),
            args,
        }
    }

    fn parse_arguments(&mut self) -> Vec<Node> {
        let mut args = Vec::new();

        loop {
            let token = self.peek();
            match token.token_type {
                TokenType::COMMA => {
                    self.skip(TokenType::COMMA);
                }
                TokenType::RPAREN => {
                    break;
                }
                _ => {
                    args.push(self.parse_expression());
                }
            }
        }

        args
    }

    fn parse_operator(&mut self) -> Operator {
        let tok = self.consume();

        match tok.token_type {
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
                panic!("Unexpected token: {}", tok.to_string());
            }
        }
    }

    fn parse_id(&mut self) -> Node {
        let token = self.consume();

        match token.token_type {
            TokenType::ID => {
                Node::Identifier(token.literal)
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
    }

    fn skip(&mut self, token_type: TokenType) {
        if self.tokens[self.position].token_type == token_type {
            self.position += 1;
        } else {
            println!("PARSED: {:?}", self.ast);
            panic!("Expected token: {:?}", token_type);
        }
    }

    fn skip_until(&mut self, token_type: TokenType) {
        while self.tokens[self.position].token_type != token_type {
            self.position += 1;
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
