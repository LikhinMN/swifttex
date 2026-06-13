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
