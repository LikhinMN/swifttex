use swifttex_lexer::{Lexer, Token};

fn collect_tokens(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let t = lexer.next_token();
        let is_eof = t == Token::EOF;
        tokens.push(t);
        if is_eof {
            break;
        }
    }
    tokens
}

#[test]
fn test_simple() {
    let tokens = collect_tokens("x^2");
    assert_eq!(
        tokens,
        vec![
            Token::Letter('x'),
            Token::Caret,
            Token::Digit('2'),
            Token::EOF,
        ]
    );
}

#[test]
fn test_fraction() {
    let tokens = collect_tokens(r"\frac{1}{2}");
    assert_eq!(
        tokens,
        vec![
            Token::Command("frac".to_string()),
            Token::LBrace,
            Token::Digit('1'),
            Token::RBrace,
            Token::LBrace,
            Token::Digit('2'),
            Token::RBrace,
            Token::EOF,
        ]
    );
}

#[test]
fn test_nested() {
    let tokens = collect_tokens(r"\frac{x^2}{y_1}");
    assert_eq!(
        tokens,
        vec![
            Token::Command("frac".to_string()),
            Token::LBrace,
            Token::Letter('x'),
            Token::Caret,
            Token::Digit('2'),
            Token::RBrace,
            Token::LBrace,
            Token::Letter('y'),
            Token::Underscore,
            Token::Digit('1'),
            Token::RBrace,
            Token::EOF,
        ]
    );
}

#[test]
fn test_greek() {
    let tokens = collect_tokens(r"\alpha + \beta");
    assert_eq!(
        tokens,
        vec![
            Token::Command("alpha".to_string()),
            Token::Whitespace,
            Token::Letter('+'),
            Token::Whitespace,
            Token::Command("beta".to_string()),
            Token::EOF,
        ]
    );
}

#[test]
fn test_edge_empty() {
    let tokens = collect_tokens("");
    assert_eq!(tokens, vec![Token::EOF]);
}

#[test]
fn test_edge_whitespace_only() {
    let tokens = collect_tokens("   \t\n ");
    assert_eq!(tokens, vec![Token::Whitespace, Token::EOF]);
}

#[test]
fn test_edge_unknown_command() {
    let tokens = collect_tokens(r"\foo");
    assert_eq!(
        tokens,
        vec![Token::Command("foo".to_string()), Token::EOF]
    );
}

#[test]
fn test_no_panic_on_repeated_input() {
    let input = "a".repeat(10000);
    let mut lexer = Lexer::new(&input);
    let mut count = 0;
    while lexer.next_token() != Token::EOF {
        count += 1;
    }
    assert_eq!(count, 10000);
}
