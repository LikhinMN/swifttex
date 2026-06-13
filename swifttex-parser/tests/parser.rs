use swifttex_parser::ast::Node;
use swifttex_parser::Parser;

fn parse(input: &str) -> Vec<Node> {
    let mut parser = Parser::new(input);
    parser.parse()
}

#[test]
fn test_superscript() {
    let nodes = parse("x^2");
    assert_eq!(
        nodes,
        vec![Node::Superscript {
            base: Box::new(Node::Text('x')),
            exp: Box::new(Node::Text('2')),
        }]
    );
}

#[test]
fn test_fraction() {
    let nodes = parse(r"\frac{1}{2}");
    assert_eq!(
        nodes,
        vec![Node::Fraction {
            numer: Box::new(Node::Group(vec![Node::Text('1')])),
            denom: Box::new(Node::Group(vec![Node::Text('2')])),
        }]
    );
}

#[test]
fn test_sqrt() {
    let nodes = parse(r"\sqrt{x}");
    assert_eq!(
        nodes,
        vec![Node::SquareRoot {
            inner: Box::new(Node::Group(vec![Node::Text('x')])),
        }]
    );
}

#[test]
fn test_fraction_with_superscript() {
    let nodes = parse(r"\frac{x^2}{y}");
    assert_eq!(
        nodes,
        vec![Node::Fraction {
            numer: Box::new(Node::Group(vec![Node::Superscript {
                base: Box::new(Node::Text('x')),
                exp: Box::new(Node::Text('2')),
            }])),
            denom: Box::new(Node::Group(vec![Node::Text('y')])),
        }]
    );
}

#[test]
fn test_greek_and_operator() {
    let nodes = parse(r"\alpha + \beta");
    assert_eq!(
        nodes,
        vec![
            Node::Symbol('α'),
            Node::Text('+'),
            Node::Symbol('β'),
        ]
    );
}

#[test]
fn test_unknown_command() {
    let nodes = parse(r"\foo");
    assert_eq!(
        nodes,
        vec![Node::Unknown("foo".to_string())]
    );
}

#[test]
fn test_empty_string() {
    let nodes = parse("");
    assert_eq!(nodes, vec![]);
}

#[test]
fn test_nested_groups() {
    let nodes = parse("{x^{2}}");
    assert_eq!(
        nodes,
        vec![Node::Group(vec![Node::Superscript {
            base: Box::new(Node::Text('x')),
            exp: Box::new(Node::Group(vec![Node::Text('2')])),
        }])]
    );
}

#[test]
fn test_matrix() {
    let nodes = swifttex_parser::Parser::new(r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}").parse();
    assert_eq!(nodes.len(), 1);
    if let swifttex_parser::ast::Node::Matrix { rows, env } = &nodes[0] {
        assert_eq!(*env, swifttex_parser::ast::MatrixEnv::Pmatrix);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
    } else {
        panic!("Expected Matrix");
    }
}

#[test]
fn test_left_right() {
    let nodes = swifttex_parser::Parser::new(r"\left( x + y \right)").parse();
    assert_eq!(nodes.len(), 1);
    if let swifttex_parser::ast::Node::Delimiter { open, close, sizing, .. } = &nodes[0] {
        assert_eq!(*open, swifttex_parser::ast::DelimChar::Paren);
        assert_eq!(*close, swifttex_parser::ast::DelimChar::Paren);
        assert_eq!(*sizing, swifttex_parser::ast::DelimSizing::Auto);
    } else {
        panic!("Expected Delimiter");
    }
}

#[test]
fn test_big_op_sum() {
    let nodes = swifttex_parser::Parser::new(r"\sum_{i=0}^{n} x_i").parse();
    if let swifttex_parser::ast::Node::BigOp { op, lower, upper } = &nodes[0] {
        assert_eq!(*op, swifttex_parser::ast::BigOpKind::Sum);
        assert!(lower.is_some());
        assert!(upper.is_some());
    } else {
        panic!("Expected BigOp");
    }
}

#[test]
fn test_big_op_int() {
    let nodes = swifttex_parser::Parser::new(r"\int_0^\infty f(x) dx").parse();
    if let swifttex_parser::ast::Node::BigOp { op, lower, upper } = &nodes[0] {
        assert_eq!(*op, swifttex_parser::ast::BigOpKind::Integral);
        assert!(lower.is_some());
        assert!(upper.is_some());
    } else {
        panic!("Expected BigOp");
    }
}

#[test]
fn test_delimiter_with_frac() {
    let nodes = swifttex_parser::Parser::new(r"\left[ \frac{1}{2} \right]").parse();
    if let swifttex_parser::ast::Node::Delimiter { open, close, .. } = &nodes[0] {
        assert_eq!(*open, swifttex_parser::ast::DelimChar::Bracket);
        assert_eq!(*close, swifttex_parser::ast::DelimChar::Bracket);
    } else {
        panic!("Expected Delimiter");
    }
}

#[test]
fn test_accent() {
    let nodes = swifttex_parser::Parser::new(r"\hat{x}").parse();
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        swifttex_parser::ast::Node::Accent { kind, .. } => {
            assert_eq!(*kind, swifttex_parser::ast::AccentKind::Hat);
        }
        _ => panic!("Expected Accent"),
    }
}

#[test]
fn test_text_op() {
    let nodes = swifttex_parser::Parser::new(r"\sin x").parse();
    assert_eq!(nodes.len(), 2);
    match &nodes[0] {
        swifttex_parser::ast::Node::TextOp(name) => {
            assert_eq!(name, "sin");
        }
        _ => panic!("Expected TextOp"),
    }
}

#[test]
fn test_text_op_subscript() {
    let nodes = swifttex_parser::Parser::new(r"\lim_{x \to 0}").parse();
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        swifttex_parser::ast::Node::Subscript { base, .. } => {
            match &**base {
                swifttex_parser::ast::Node::TextOp(name) => assert_eq!(name, "lim"),
                _ => panic!("Expected TextOp base"),
            }
        }
        _ => panic!("Expected Subscript"),
    }
}

#[test]
fn test_spacing() {
    let nodes = swifttex_parser::Parser::new(r"x \, y").parse();
    assert_eq!(nodes.len(), 3);
    match &nodes[1] {
        swifttex_parser::ast::Node::Spacing(em) => {
            assert!((*em - 0.167).abs() < 1e-6);
        }
        _ => panic!("Expected Spacing"),
    }
}

#[test]
fn test_symbol_infty() {
    let nodes = swifttex_parser::Parser::new(r"\infty").parse();
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        swifttex_parser::ast::Node::Symbol(c) => assert_eq!(*c, '∞'),
        _ => panic!("Expected Symbol"),
    }
}

#[test]
fn test_symbol_leq() {
    let nodes = swifttex_parser::Parser::new(r"x \leq y").parse();
    assert_eq!(nodes.len(), 3);
    match &nodes[1] {
        swifttex_parser::ast::Node::Symbol(c) => assert_eq!(*c, '≤'),
        _ => panic!("Expected Symbol"),
    }
}

#[test]
fn test_forall_in() {
    let nodes = swifttex_parser::Parser::new(r"\forall x \in \mathbb{R}").parse();
    assert!(nodes.len() > 0);
}
