use crate::ast;
use crate::ast::Node;
use crate::lexer;
use crate::parser;

#[test]
fn test_let_statements() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ";
    let lexer: lexer::Lexer = lexer::Lexer::new(String::from(input));
    let mut parser: parser::Parser = parser::Parser::new(lexer);

    let program: ast::Program = parser.parse_program().expect("Failed to parse program");

    check_parser_errors(&parser);

    assert_eq!(program.statements.len(), 3);

    let checks: Vec<&str> = vec!["x", "y", "foobar"];

    for (i, c) in checks.iter().enumerate() {
        assert!(test_let_statement(&program.statements[i], c));
    }
}

fn test_let_statement(statement: &ast::Statement, name: &str) -> bool {
    assert_eq!(statement.token_literal(), String::from("let"));

    match statement {
        ast::Statement::Let(l) => {
            assert_eq!(l.name.value, name);
            assert_eq!(l.name.token_literal(), name);
        }
        _ => {
            println!("ERROR: not a let statement in let statement test");
            return false;
        }
    }

    true
}

fn check_parser_errors(parser: &parser::Parser) {
    if parser.errors.len() == 0 {
        return;
    }
    println!("parser had {} errors", parser.errors.len());

    for error in &parser.errors {
        println!("{}", error);
    }

    assert!(false);
}

#[test]
fn test_return_statements() {
    let input = "
        return 5;
        return 8;
        return 91230;
    ";
    let lexer: lexer::Lexer = lexer::Lexer::new(String::from(input));
    let mut parser: parser::Parser = parser::Parser::new(lexer);

    let program: ast::Program = parser.parse_program().expect("Failed to parse program");

    check_parser_errors(&parser);

    assert_eq!(program.statements.len(), 3);

    for statement in program.statements {
        match statement {
            ast::Statement::Return(_) => {}
            ref s => {
                assert!(false, "Expected RETURN statement got {} statement", s);
            }
        }
        if statement.token_literal() != "return" {
            assert!(
                false,
                "Expected return literal got {} literal",
                statement.token_literal()
            );
        }
    }
}

#[test]
fn test_ident_expression() {
    let input = "foobar;";
    let lexer: lexer::Lexer = lexer::Lexer::new(String::from(input));
    let mut parser: parser::Parser = parser::Parser::new(lexer);

    let program: ast::Program = parser.parse_program().expect("failed to parse program");

    check_parser_errors(&parser);

    if program.statements.len() != 1 {
        assert!(false, "program does not have enough statements");
    }

    match &program.statements[0] {
        ast::Statement::Expression(s) => match s.expression.as_ref().unwrap() {
            ast::Expression::Identifier(e) => {
                if e.value != "foobar" {
                    assert!(false, "identifier not foobar, got {}", e.value);
                }
                if e.token_literal() != "foobar" {
                    assert!(false, "literal not foobar, got {}", e.token_literal());
                }
            }

            e => assert!(false, "expression not an identifier, got {}", e),
        },
        s => assert!(false, "not an expression statement, got {}", s),
    }
}
