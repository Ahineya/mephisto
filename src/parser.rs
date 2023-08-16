use crate::lexer::token::Token;
use crate::lexer::token_type::TokenType;
use crate::parser::Node::Stub;

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
    Identifier(String),
    ExpressionStmt {
        child: Box<Node>,
    },
    AssignmentExpr {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },

    VariableDeclarationStmt {
        id: Box<Node>,
        initializer: Box<Node>,
        specifier: VariableSpecifier,
    },

    MemberExpr {
        object: Box<Node>,
        property: Box<Node>,
    },

    Stub,

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
                    TokenType::PROCESS => {
                        children.push(self.parse_process())
                    }
                    TokenType::BLOCK => {
                        children.push(self.parse_block())
                    }
                    TokenType::INPUT | TokenType::OUTPUT | TokenType::BUFFER | TokenType::LET | TokenType::CONST => {
                        children.push(self.parse_variable_declaration_stmt())
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

    fn parse_statement(&mut self) -> Node {
        let token = self.peek();

        match token.token_type {
            TokenType::ID => {
                Node::ExpressionStmt { child: Box::new(self.parse_assignment_expression()) }
            }
            _ => {
                panic!("Unexpected token: {}", token.to_string());
            }
        }
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

    fn parse_variable_specifier(&mut self) -> VariableSpecifier {
        let token = self.consume();

        match token.token_type {
            TokenType::INPUT => {
                VariableSpecifier::Input
            }
            TokenType::OUTPUT => {
                VariableSpecifier::Output
            }
            TokenType::BUFFER => {
                VariableSpecifier::Buffer
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
        let initializer = self.parse_expr();
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

        let expr = self.parse_expr();

        println!("id: {:?}, expr: {:?}", id, expr);

        self.skip(TokenType::SEMI);

        Node::AssignmentExpr {
            lhs: Box::new(id),
            rhs: Box::new(expr),
        }
    }

    fn parse_expr(&mut self) -> Node {
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
                    child: Box::new(self.parse_expr()),
                }
            }
            TokenType::MINUS => {
                Node::UnaryExpr {
                    op: Operator::Minus,
                    child: Box::new(self.parse_expr()),
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
                let expr = self.parse_expr();
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
                    args.push(self.parse_expr());
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
