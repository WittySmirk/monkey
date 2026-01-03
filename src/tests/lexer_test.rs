use crate::lexer;
use crate::token;

struct Test {
    expected_type: token::TokenType,
    expected_literal: &'static str,
}

#[test]
fn test_next_token() {
    let input = "=+(){},;";
    let checks: Vec<Test> = vec![
        Test {
            expected_type: token::TokenType::ASSIGN,
            expected_literal: "=",
        },
        Test {
            expected_type: token::TokenType::PLUS,
            expected_literal: "+",
        },
        Test {
            expected_type: token::TokenType::LPARAN,
            expected_literal: "LPARAN",
        },
        Test {
            expected_type: token::TokenType::RPARAN,
            expected_literal: "RPARAN",
        },
        Test {
            expected_type: token::TokenType::LBRACE,
            expected_literal: "{",
        },
        Test {
            expected_type: token::TokenType::RBRACE,
            expected_literal: "R",
        },
        Test {
            expected_type: token::TokenType::COMMA,
            expected_literal: ",",
        },
        Test {
            expected_type: token::TokenType::SEMICOLON,
            expected_literal: ";",
        },
        Test {
            expected_type: token::TokenType::EOF,
            expected_literal: "",
        },
    ];

    let l = lexer::Lexer::new(input);

    for t in checks {
        let curr = l.next_token();

        // Todo: document this better
        assert_eq!(curr, t.expected_type);
        assert_eq!(curr.as_str(), t.expected_literal);
    }
}
